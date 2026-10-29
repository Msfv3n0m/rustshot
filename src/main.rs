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

fn rejoin_quoted_commands(args: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut accumulator = String::new();
    let mut in_single_quote = false;

    for arg in args {
        if in_single_quote {
            accumulator.push(' ');
            if let Some(stripped) = arg.strip_suffix('\'') {
                accumulator.push_str(stripped);
                result.push(std::mem::take(&mut accumulator));
                in_single_quote = false;
            } else {
                accumulator.push_str(&arg);
            }
        } else if let Some(rest) = arg.strip_prefix('\'') {
            if let Some(inner) = rest.strip_suffix('\'') {
                result.push(inner.to_string());
            } else {
                in_single_quote = true;
                accumulator = rest.to_string();
            }
        } else {
            result.push(arg);
        }
    }

    if !accumulator.is_empty() {
        result.push(accumulator);
    }

    result
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let theme = Theme::default();
    let commands = rejoin_quoted_commands(cli.commands);

    let results: Vec<CommandResult> = if commands.is_empty() {
        if std::io::stdin().is_terminal() {
            anyhow::bail!("No commands provided. Usage: rustshot -- \"command\" or pipe input via stdin.");
        }
        vec![executor::read_stdin()?]
    } else if let Some(ref device) = cli.device {
        commands
            .iter()
            .map(|cmd| {
                eprintln!("Executing on device {}: {}", device, cmd);
                executor::execute_adb_command(cmd, device, cli.columns, cli.rows)
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        commands
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

    let scale: u32 = 2;

    let font_size = cli.font_size.unwrap_or_else(|| {
        compute_font_size(
            total_rows,
            max_cols,
            cli.padding * scale,
            cli.margin * scale,
            if decoration {
                theme.title_bar_height * scale
            } else {
                0
            },
            1200 * scale,
            1600 * scale,
        )
    }) * scale as f32;

    let font_config = load_fonts(font_size);
    let scaled_theme = theme.scaled(scale);

    let img = render(
        &panels,
        show_cmd,
        &scaled_theme,
        &font_config,
        cli.padding * scale,
        cli.margin * scale,
        decoration,
        shadow,
    );

    img.save(&cli.filename)
        .with_context(|| format!("Failed to save image to {:?}", cli.filename))?;

    eprintln!("Saved to {:?}", cli.filename);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_quoted_command_is_rejoined() {
        let args = vec!["'echo".into(), "hello".into(), "world'".into()];
        assert_eq!(rejoin_quoted_commands(args), vec!["echo hello world"]);
    }

    #[test]
    fn fully_single_quoted_single_arg() {
        let args = vec!["'echo hello world'".into()];
        assert_eq!(rejoin_quoted_commands(args), vec!["echo hello world"]);
    }

    #[test]
    fn unquoted_commands_unchanged() {
        let args = vec!["echo hello".into(), "git status".into()];
        assert_eq!(
            rejoin_quoted_commands(args),
            vec!["echo hello", "git status"]
        );
    }

    #[test]
    fn mixed_quoted_and_unquoted() {
        let args = vec![
            "'git".into(),
            "log".into(),
            "--oneline'".into(),
            "echo hi".into(),
        ];
        assert_eq!(
            rejoin_quoted_commands(args),
            vec!["git log --oneline", "echo hi"]
        );
    }

    #[test]
    fn unclosed_single_quote_still_collected() {
        let args = vec!["'echo".into(), "hello".into()];
        assert_eq!(rejoin_quoted_commands(args), vec!["echo hello"]);
    }

    #[test]
    fn multiple_single_quoted_commands() {
        let args = vec![
            "'echo".into(),
            "first'".into(),
            "'echo".into(),
            "second'".into(),
        ];
        assert_eq!(
            rejoin_quoted_commands(args),
            vec!["echo first", "echo second"]
        );
    }
}
