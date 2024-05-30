mod arguments;

use clap::{Arg};
use serde::Deserialize;
use std::fs;
use std::fs::FileType;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use walkdir::{DirEntry, WalkDir};
use log::{info, error, warn};
use flexi_logger::{Logger, Duplicate, FileSpec, WriteMode};

#[derive(Debug, Deserialize)]
struct Stream {
    codec_name: Option<String>,
    pix_fmt: Option<String>,
    field_order: Option<String>,
    codec_type: Option<String>,
    index: Option<usize>,
    bit_rate: Option<String>,
    channels: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct FfprobeOutput {
    streams: Vec<Stream>,
}

fn run_ffprobe_command(input_file: &str) -> Result<FfprobeOutput, String> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "error",
            "-show_entries", "stream=index,codec_type,codec_name,pix_fmt,field_order,channels,bit_rate",
            "-of", "json", input_file,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    if output.status.success() {
        serde_json::from_slice(&output.stdout).map_err(|e| format!("Error decoding JSON from ffprobe for {}: {}", input_file, e))
    } else {
        Err(format!(
            "Error running ffprobe for {}: {}",
            input_file,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn convert_to_mkv(
    input_file: &str,
    output_file: &str,
    pix_fmt: &str,
    field_order: &str,
    interlaced_overwrite: bool,
    encoder: &str,
    preset: &str,
    crf: u8,
    audio_file_types: &[String],
    audio_codec: &str
) -> Result<ExitStatus, String> {
    let ffprobe_output = run_ffprobe_command(input_file)?;

    let interlaced = field_order != "progressive" || interlaced_overwrite;

    let audio_tracks: Vec<&Stream> = ffprobe_output
        .streams
        .iter()
        .filter(|stream| stream.codec_type == Some("audio".to_string()))
        .collect();

    let subtitle_tracks: Vec<&Stream> = ffprobe_output
        .streams
        .iter()
        .filter(|stream| stream.codec_type == Some("subtitle".to_string()))
        .collect();

    let mut command = Command::new("ffmpeg");
    command.arg("-i").arg(input_file);
    command.arg("-c:v").arg(encoder);
    command.arg("-preset").arg(preset);
    command.arg("-crf").arg(crf.to_string());
    command.arg("-pix_fmt").arg(pix_fmt);
    command.arg("-map").arg("0:v:0");

    if interlaced {
        command.arg("-vf").arg("yadif");
    }

    for track in audio_tracks {
        let codec = track.codec_name.as_deref().unwrap_or("");
        command.arg("-map").arg(format!("0:{}", track.index.unwrap_or_default()));
        if audio_file_types.contains(&codec.to_string()) {
            command.arg("-c:a").arg("copy");
        } else {
            command.arg("-c:a").arg(audio_codec);
        }
        if let Some(bit_rate) = &track.bit_rate {
            command.arg("-b:a").arg(bit_rate);
        }
        if let Some(channels) = track.channels {
            command.arg("-ac").arg(channels.to_string());
        }
    }

    for track in subtitle_tracks {
        command.arg("-map").arg(format!("0:{}", track.index.unwrap_or_default()));
        command.arg("-c:s").arg("copy");
    }

    command.arg(output_file);

    command.status().map_err(|e| format!("Failed to execute ffmpeg: {}", e))
}

fn traverse_and_convert(
    source_dir: &str,
    output_dir: &str,
    interlaced_overwrite: bool,
    video_file_types: Vec<String>,
    encoder: &str,
    preset: &str,
    crf: u8,
    audio_file_types: Vec<String>,
    audio_codec: &str,
    skip_codecs: Vec<String>,)-> Result<(), String> {
    let mut original_total_size = 0u64;
    let mut converted_total_size = 0u64;

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter(|e| correct_file_type(e.as_ref().unwrap().path(), &video_file_types)) {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            let input_file = path.to_str().ok_or_else(|| "Failed to convert path to string".to_string())?;
            let relative_path = path.strip_prefix(source_dir).map_err(|e| format!("Failed to get relative path: {}", e))?;
            let output_path = Path::new(output_dir).join(relative_path);
            let output_file = output_path.with_extension("mkv");
            fs::create_dir_all(output_file.parent().ok_or_else(|| "Failed to get output directory parent".to_string())?)
                .map_err(|e| format!("Failed to create output directory: {}", e))?;

            if output_file.exists() {
                warn!("Skipping {} as the output file already exists", input_file);
                continue;
            }

            let video_info = run_ffprobe_command(input_file)?;
            let video_stream = video_info.streams.iter().find(|stream| stream.codec_type == Some("video".to_string()));

            if let Some(video_stream) = video_stream {
                if !skip_codecs.contains(&video_stream.codec_name.as_deref().unwrap_or("").to_string()) {
                    let pix_fmt = video_stream.pix_fmt.as_deref().unwrap_or("yuv420p");
                    let field_order = video_stream.field_order.as_deref().unwrap_or("progressive");

                    info!("{:?}\nvideo_codec_name: {:?}\npix_fmt: {:?}\nfield_order: {:?}",
                        input_file,
                        video_stream.codec_name.as_deref().unwrap_or("unknown"),
                        pix_fmt,
                        field_order
                    );

                    let original_size = fs::metadata(input_file).map_err(|e| format!("Failed to get file metadata: {}", e))?.len();
                    original_total_size += original_size;

                    convert_to_mkv(
                        input_file,
                        output_file.to_str().ok_or_else(|| "Failed to convert output path to string".to_string())?,
                        pix_fmt,
                        field_order,
                        interlaced_overwrite,
                        encoder,
                        preset,
                        crf,
                        &audio_file_types,
                        audio_codec
                    )?;
                    let converted_size = fs::metadata(&output_file).map_err(|e| format!("Failed to get file metadata: {}", e))?.len();
                    converted_total_size += converted_size;

                    info!("\nOriginal size: {:.2} MB\nConverted size: {:.2} MB\nSpace saved: {:.2} MB\nPercentage saved: {:.2}%"
                        , original_size as f64 / (1024.0 * 1024.0)
                        , converted_size as f64 / (1024.0 * 1024.0)
                        , (original_size as f64 - converted_size as f64) / (1024.0 * 1024.0)
                        , ((original_size as f64 - converted_size as f64) as f64 / original_size as f64) * 100.0);
                } else {
                    warn!("Skipping {} as it is already in {} format", input_file, video_stream.codec_name.as_deref().unwrap_or(""));
                }
            }
        }
    }
    
    info!("\nTotal original size: {:.2} MB\nTotal converted size: {:.2} MB\nTotal space saved: {:.2} MB\nTotal percentage saved: {:.2}%"
                        , original_total_size as f64 / (1024.0 * 1024.0)
                        , converted_total_size as f64 / (1024.0 * 1024.0)
                        , (original_total_size as f64 - converted_total_size as f64) / (1024.0 * 1024.0)
                        , ((original_total_size as f64 - converted_total_size as f64) as f64 / original_total_size as f64) * 100.0);

    Ok(())
}

fn correct_file_type(path: &Path, file_types: &Vec<String>) -> bool {
    match path.extension() {
        Some(ext) => file_types.iter().any(|t| ext.to_str().unwrap_or("") == t),
        None => false,
    }
}

fn main() {
    // Initialize logging
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("log")
                .basename("application")
                .suffix("log"),
        )
        .duplicate_to_stdout(Duplicate::All)
        .write_mode(WriteMode::BufferAndFlush)
        .format_for_files(flexi_logger::detailed_format)
        .start()
        .unwrap();

    let args = arguments::get_arguments();

    match traverse_and_convert(
        &args.source,
        &args.output,
        args.interlace_overwrite,
        args.video_file_types,
        &args.encoder,
        &args.preset,
        args.crf,
        args.ok_audio_codec,
        &args.audio_codec,
        args.skip_video_codecs,) {
        Ok(_) => {info!("All done!")}
        Err(e) => {error!("Error: {}", e);}
    };
}