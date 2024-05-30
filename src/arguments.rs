use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Source directory
    #[arg(short, long)]
    pub source: String,

    /// Output directory
    #[arg(short, long)]
    pub output: String,

    /// If all input should be treated as interlaces (use when auto-detection fails)
    #[arg(short, long, default_value_t = false)]
    pub interlace_overwrite: bool,

    /// Video file types that are considered for conversion
    #[arg(short, long, default_values = &["mp4", "avi", "mov", "mkv"])]
    pub video_file_types: Vec<String>,

    /// Encoder to use for video conversion
    #[arg(short, long, default_value = "hevc_nvenc")]
    pub encoder: String,

    /// Preset for the encoder
    #[arg(short, long, default_value = "slow")]
    pub preset: String,

    /// CRF (Constant Rate Factor) value for the encoder
    #[arg(short, long, default_value_t = 18)]
    pub crf: u8,

    /// Audio codecs that are considered for conversion
    #[arg(short, long, default_values = &["flac", "aac", "ac3", "eac3"])]
    pub convert_audio_codec: Vec<String>,

    /// Audio codec to convert to
    #[arg(short, long, default_value = "aac")]
    pub audio_codec: String,

    /// Video codecs to skip
    #[arg(short, long, default_values = &["hevc", "h264"])]
    pub skip_video_codecs: Vec<String>,
}

pub fn get_arguments() -> Args {
    return Args::parse();
}