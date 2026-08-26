use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rustshot",
    version,
    about = "Capture terminal command output as beautiful PNG images",
    after_help = "EXAMPLES:\n  \
        rustshot -- \"echo hello\"\n  \
        rustshot -- \"git status\" \"git log --oneline -5\"\n  \
        rustshot -f screenshot.png -- \"echo first\" \"echo second\" \"echo third\"\n  \
        rustshot --decoration --shadow -m 20 -- \"echo styled\"\n  \
        nmap -sV 10.0.0.1 | rustshot -f scan.png"
)]
pub struct Cli {
    /// Commands to execute; pass each as a quoted string after --
    #[arg(last = true)]
    pub commands: Vec<String>,

    /// Output filename
    #[arg(short = 'f', long, default_value = "out.png")]
    pub filename: PathBuf,

    /// Show the executed command above its output
    #[arg(short = 'c', long, default_value_t = true)]
    pub show_cmd: bool,

    /// Hide the executed command above its output
    #[arg(long)]
    pub hide_cmd: bool,

    /// Terminal columns for PTY
    #[arg(short = 'C', long, default_value_t = 80)]
    pub columns: u16,

    /// Terminal rows for PTY
    #[arg(long, default_value_t = 500)]
    pub rows: u16,

    /// Padding inside window frame in pixels
    #[arg(short, long, default_value_t = 16)]
    pub padding: u32,

    /// Margin outside window in pixels
    #[arg(short, long, default_value_t = 0)]
    pub margin: u32,

    /// Enable window chrome (title bar, traffic lights)
    #[arg(long)]
    pub decoration: bool,

    /// Enable drop shadow
    #[arg(long)]
    pub shadow: bool,

    /// Force a specific font size (disables dynamic sizing)
    #[arg(long)]
    pub font_size: Option<f32>,
}
