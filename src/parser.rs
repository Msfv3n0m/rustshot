use anyhow::Result;

use crate::executor::CommandResult;

#[derive(Clone, Debug)]
pub enum CellColor {
    Default,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

#[derive(Clone, Debug)]
pub struct StyledCell {
    pub ch: String,
    pub fg: CellColor,
    pub bg: CellColor,
    pub bold: bool,
    #[allow(dead_code)]
    pub italic: bool,
    pub underline: bool,
    pub inverse: bool,
}

pub struct ParsedOutput {
    pub command: String,
    pub grid: Vec<Vec<StyledCell>>,
    pub rows: usize,
    pub cols: usize,
}

fn convert_color(color: vt100::Color) -> CellColor {
    match color {
        vt100::Color::Default => CellColor::Default,
        vt100::Color::Idx(idx) => CellColor::Indexed(idx),
        vt100::Color::Rgb(r, g, b) => CellColor::Rgb(r, g, b),
    }
}

pub fn parse_ansi(result: &CommandResult, pty_cols: u16, pty_rows: u16) -> Result<ParsedOutput> {
    let mut parser = vt100::Parser::new(pty_rows, pty_cols, 0);
    parser.process(&result.raw_output);

    let screen = parser.screen();
    let (rows, cols) = screen.size();

    let mut grid: Vec<Vec<StyledCell>> = Vec::new();

    for row in 0..rows {
        let mut line: Vec<StyledCell> = Vec::new();
        let mut col = 0u16;
        while col < cols {
            let cell = screen.cell(row, col);
            match cell {
                Some(cell) => {
                    if cell.is_wide_continuation() {
                        col += 1;
                        continue;
                    }
                    let contents = cell.contents();
                    let ch = if contents.is_empty() {
                        " ".to_string()
                    } else {
                        contents
                    };
                    line.push(StyledCell {
                        ch,
                        fg: convert_color(cell.fgcolor()),
                        bg: convert_color(cell.bgcolor()),
                        bold: cell.bold(),
                        italic: cell.italic(),
                        underline: cell.underline(),
                        inverse: cell.inverse(),
                    });
                }
                None => {
                    line.push(StyledCell {
                        ch: " ".to_string(),
                        fg: CellColor::Default,
                        bg: CellColor::Default,
                        bold: false,
                        italic: false,
                        underline: false,
                        inverse: false,
                    });
                }
            }
            col += 1;
        }
        grid.push(line);
    }

    let mut actual_rows = grid.len();
    while actual_rows > 0 {
        let row = &grid[actual_rows - 1];
        let all_blank = row.iter().all(|c| c.ch.trim().is_empty() && matches!(c.bg, CellColor::Default));
        if all_blank {
            actual_rows -= 1;
        } else {
            break;
        }
    }
    grid.truncate(actual_rows);

    let actual_cols = grid
        .iter()
        .map(|row| {
            let mut last_non_blank = 0;
            for (i, cell) in row.iter().enumerate() {
                if !cell.ch.trim().is_empty() || !matches!(cell.bg, CellColor::Default) {
                    last_non_blank = i + 1;
                }
            }
            last_non_blank
        })
        .max()
        .unwrap_or(0);

    Ok(ParsedOutput {
        command: result.command.clone(),
        grid,
        rows: actual_rows,
        cols: actual_cols,
    })
}
