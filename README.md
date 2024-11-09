# 🎬 Media Converter Tool

Welcome to the Media Converter Tool! This handy utility is designed to help you convert your media files to more modern and compatible formats, ensuring smooth playback on all your devices. Whether you're using Jellyfin, Plesk, or any other media server, this tool will make sure your media library is up-to-date and compatible with all your clients, including older devices.

## 🚀 Features

- Convert video files to modern formats like H.264 and HEVC.
- Automatically detect and handle interlaced video content.
- Preserve or convert audio tracks to your preferred format.
- Skip already compatible video codecs to save time.
- Detailed logging to both console and files for easy troubleshooting.

## 🛠️ Usage

Here's how to use the Media Converter Tool. You can customize various options to suit your needs.

### Command Line Options

| Option                      | Description                                                                                   | Default Values                                  |
|-----------------------------|-----------------------------------------------------------------------------------------------|-------------------------------------------------|
| `-s`, `--source`            | Source directory containing the media files to be converted.                                  | Required                                        |
| `-o`, `--output`            | Output directory where the converted files will be saved.                                     | Required                                        |
| `-i`, `--interlace-overwrite` | Treat all input as interlaced video (use when auto-detection fails).                        | `false`                                         |
| `-v`, `--video-file-types`  | Video file types to be considered for conversion.                                             | `mp4`, `avi`, `mov`, `mkv`                      |
| `-e`, `--encoder`           | Encoder to use for video conversion.                                                          | `hevc_nvenc`                                    |
| `-p`, `--preset`            | Preset for the encoder.                                                                       | `slow`                                          |
| `-c`, `--crf`               | CRF (Constant Rate Factor) value for the encoder.                                             | `18`                                            |
| `-a`, `--ok-audio-codec`    | Audio codecs that are not considered for conversion:                                          | `flac`, `mp3`, `ac3`, `aac`                     |
| `-A`, `--audio-codec`       | Audio codec to convert to.                                                                    | `aac`                                           |
| `-k`, `--skip-video-codecs` | Video codecs to skip (e.g., already compatible codecs that don't need conversion).            | `hevc`, `h264`                                  |

### Example Usage

Convert all your media files in the `/media/source` directory to be compatible with older devices and save the converted files to `/media/output`:

```sh
cargo run -- --source /media/source --output /media/output --interlace-overwrite --video-file-types mp4 avi mov mkv --encoder hevc_nvenc --preset slow --crf 18 --ok-audio-codec flac aac ac3 eac3 --audio-codec aac --skip-video-codecs hevc h264 av1
```

## Handy Use Case
For media servers like Jellyfin or Plesk, it's essential to have your media files in formats that all devices can easily stream. Older media files in formats like MPEG2 might not be compatible with all clients. Using this tool, you can convert your media files to H.264 or HEVC, ensuring compatibility and smooth streaming across all devices.

## 📄 Logging
The tool logs detailed information about the conversion process to both the console and log files:

log/application.log: General logs for the conversion process.
log/error.log: Detailed error logs for troubleshooting.

## 📦 Installation

### Installing ffmpeg

#### Windows

You can install `ffmpeg` on Windows using either direct download or `choco` (Chocolatey).

1. **Direct Download**:
   - Go to the [ffmpeg download page](https://ffmpeg.org/download.html).
   - Download the latest version for Windows.
   - Extract the downloaded files to a directory (e.g., `C:\ffmpeg`).
   - Add the `bin` directory (e.g., `C:\ffmpeg\bin`) to your system's PATH environment variable.

2. **Using Chocolatey**:
   - Open Command Prompt or PowerShell as an administrator.
   - Run the following command to install `ffmpeg`:
     ```sh
     choco install ffmpeg
     ```

#### Linux

On Linux, you can install `ffmpeg` using your distribution's package manager.

- **Debian/Ubuntu**:
  ```sh
  sudo apt update
  sudo apt install ffmpeg
  ```

- **CentOS/RHEL**:
  ```sh
  sudo yum install epel-release
  sudo yum install ffmpeg
  ```

- **Fedora**:
  ```sh
  sudo dnf install ffmpeg
  ```

### Installing the Media Converter Tool

To install the Media Converter Tool, ensure you have Rust installed and then clone this repository:

```sh
git clone https://github.com/yourusername/mediaconverter.git
cd mediaconverter
cargo build --release
```
## 🙌 Contributions
Contributions are welcome! Feel free to open issues or submit pull requests to improve the Media Converter Tool.

Happy converting! 🎉
