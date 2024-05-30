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
    
    ///if all input should be treated as interlaces (use when auto-detection fails)
    #[arg(short, long, default_value_t = false)]
    pub interlace_overwrite: bool,
    
    ///file types that are considered for conversion
    #[arg(short, long, default_values = &["mp4", "avi", "mov", "mkv"])]
    pub file_types: Vec<String>,
}

pub fn get_arguments() -> Args {
    return Args::parse();
}