mod cli;
mod executor;
mod font;
mod parser;
mod renderer;
mod theme;

use std::io::IsTerminal;

use anyhow::{Context, Result};
use clap::Parser;

use cli::Cli;
use executor::CommandResult;
use font::{compute_font_size, load_fonts};
use parser::parse_ansi;
use renderer::render;
use theme::Theme;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let theme = Theme::default();

    let results: Vec<CommandResult> = if cli.commands.is_empty() {
        if std::io::stdin().is_terminal() {
            anyhow::bail!("No commands provided. Usage: rustshot -- \"command\" or pipe input via stdin.");
        }
        vec![executor::read_stdin()?]
    } else {
        cli.commands
            .iter()
            .map(|cmd| {
                eprintln!("Executing: {}", cmd);
                executor::execute_command(cmd, cli.columns, cli.rows)
            })
            .collect::<Result<Vec<_>>>()?
    };

    let panels: Vec<parser::ParsedOutput> = results
        .iter()
        .map(|r| parse_ansi(r, cli.columns, cli.rows))
        .collect::<Result<Vec<_>>>()?;

    let total_rows: usize = panels.iter().map(|p| p.rows).sum();
    let max_cols: usize = panels.iter().map(|p| p.cols).max().unwrap_or(80);

    let decoration = cli.decoration;
    let shadow = cli.shadow;
    let show_cmd = cli.show_cmd && !cli.hide_cmd;

    let font_size = cli.font_size.unwrap_or_else(|| {
        compute_font_size(
            total_rows,
            max_cols,
            cli.padding,
            cli.margin,
            if decoration {
                theme.title_bar_height
            } else {
                0
            },
            1200,
            1600,
        )
    });

    let font_config = load_fonts(font_size);

    let img = render(
        &panels,
        show_cmd,
        &theme,
        &font_config,
        cli.padding,
        cli.margin,
        decoration,
        shadow,
    );

    img.save(&cli.filename)
        .with_context(|| format!("Failed to save image to {:?}", cli.filename))?;

    eprintln!("Saved to {:?}", cli.filename);

    Ok(())
}
