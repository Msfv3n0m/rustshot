use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustshot", version, about = "Capture terminal command output as beautiful PNG images")]
pub struct Cli {
    /// Commands to execute (pass after --)
    #[arg(last = true)]
    pub commands: Vec<String>,

    /// Output filename
    #[arg(short = 'f', long, default_value = "out.png")]
    pub filename: PathBuf,

    /// Show the executed command above its output
    #[arg(short = 'c', long)]
    pub show_cmd: bool,

    /// Terminal columns for PTY
    #[arg(short = 'C', long, default_value_t = 80)]
    pub columns: u16,

    /// Terminal rows for PTY
    #[arg(long, default_value_t = 24)]
    pub rows: u16,

    /// Padding inside window frame in pixels
    #[arg(short, long, default_value_t = 16)]
    pub padding: u32,

    /// Margin outside window in pixels
    #[arg(short, long, default_value_t = 20)]
    pub margin: u32,

    /// Disable window chrome (title bar, rounded corners)
    #[arg(long)]
    pub no_decoration: bool,

    /// Disable drop shadow
    #[arg(long)]
    pub no_shadow: bool,

    /// Force a specific font size (disables dynamic sizing)
    #[arg(long)]
    pub font_size: Option<f32>,
}
