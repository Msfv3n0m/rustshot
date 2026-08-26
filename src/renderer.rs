use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

use crate::font::FontConfig;
use crate::parser::{CellColor, ParsedOutput};
use crate::theme::{resolve_color, Theme};

struct PanelLayout {
    y_offset: u32,
    cmd_header_height: u32,
    _content_height: u32,
}

fn compute_layout(
    panels: &[ParsedOutput],
    show_cmd: bool,
    font_config: &FontConfig,
) -> (Vec<PanelLayout>, u32) {
    let mut layouts = Vec::new();
    let mut y = 0u32;

    for panel in panels.iter() {
        let cmd_h = if show_cmd && !panel.command.is_empty() {
            font_config.cell_height as u32
        } else {
            0
        };
        let content_h = panel.rows as u32 * font_config.cell_height as u32;

        layouts.push(PanelLayout {
            y_offset: y,
            cmd_header_height: cmd_h,
            _content_height: content_h,
        });
        y += cmd_h + content_h;
    }

    (layouts, y)
}

fn draw_rounded_rect_filled(
    img: &mut RgbaImage,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    radius: u32,
    color: Rgba<u8>,
) {
    if radius == 0 || h < 2 * radius || w < 2 * radius {
        draw_filled_rect_mut(img, Rect::at(x, y).of_size(w, h), color);
        return;
    }

    // Center body
    draw_filled_rect_mut(
        img,
        Rect::at(x, y + radius as i32).of_size(w, h - 2 * radius),
        color,
    );
    // Top strip
    draw_filled_rect_mut(
        img,
        Rect::at(x + radius as i32, y).of_size(w - 2 * radius, radius),
        color,
    );
    // Bottom strip
    draw_filled_rect_mut(
        img,
        Rect::at(x + radius as i32, y + (h - radius) as i32).of_size(w - 2 * radius, radius),
        color,
    );
    // Four corner circles
    let r = radius as i32;
    draw_filled_circle_mut(img, (x + r, y + r), r, color);
    draw_filled_circle_mut(img, (x + w as i32 - r - 1, y + r), r, color);
    draw_filled_circle_mut(img, (x + r, y + h as i32 - r - 1), r, color);
    draw_filled_circle_mut(img, (x + w as i32 - r - 1, y + h as i32 - r - 1), r, color);
}

fn alpha_blend(bg: Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
    let a = fg.0[3] as f32 / 255.0;
    let inv_a = 1.0 - a;
    Rgba([
        (fg.0[0] as f32 * a + bg.0[0] as f32 * inv_a) as u8,
        (fg.0[1] as f32 * a + bg.0[1] as f32 * inv_a) as u8,
        (fg.0[2] as f32 * a + bg.0[2] as f32 * inv_a) as u8,
        255,
    ])
}

#[allow(clippy::too_many_arguments)]
fn draw_shadow(img: &mut RgbaImage, x: i32, y: i32, w: u32, h: u32, offset: u32, radius: u32, color: Rgba<u8>) {
    let sx = x + offset as i32;
    let sy = y + offset as i32;

    let shadow_w = w;
    let shadow_h = h;

    for py in 0..shadow_h {
        for px in 0..shadow_w {
            let ix = sx + px as i32;
            let iy = sy + py as i32;
            if ix >= 0 && iy >= 0 && (ix as u32) < img.width() && (iy as u32) < img.height() {
                let inside = is_inside_rounded_rect(px, py, shadow_w, shadow_h, radius);
                if inside {
                    let existing = *img.get_pixel(ix as u32, iy as u32);
                    let blended = alpha_blend(existing, color);
                    img.put_pixel(ix as u32, iy as u32, blended);
                }
            }
        }
    }
}

fn is_inside_rounded_rect(px: u32, py: u32, w: u32, h: u32, r: u32) -> bool {
    if r == 0 {
        return true;
    }
    // Check if in corner regions
    let in_left = px < r;
    let in_right = px >= w - r;
    let in_top = py < r;
    let in_bottom = py >= h - r;

    if in_left && in_top {
        let dx = r as f32 - px as f32 - 0.5;
        let dy = r as f32 - py as f32 - 0.5;
        dx * dx + dy * dy <= (r as f32) * (r as f32)
    } else if in_right && in_top {
        let dx = px as f32 - (w - r) as f32 + 0.5;
        let dy = r as f32 - py as f32 - 0.5;
        dx * dx + dy * dy <= (r as f32) * (r as f32)
    } else if in_left && in_bottom {
        let dx = r as f32 - px as f32 - 0.5;
        let dy = py as f32 - (h - r) as f32 + 0.5;
        dx * dx + dy * dy <= (r as f32) * (r as f32)
    } else if in_right && in_bottom {
        let dx = px as f32 - (w - r) as f32 + 0.5;
        let dy = py as f32 - (h - r) as f32 + 0.5;
        dx * dx + dy * dy <= (r as f32) * (r as f32)
    } else {
        true
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render(
    panels: &[ParsedOutput],
    show_cmd: bool,
    theme: &Theme,
    font_config: &FontConfig,
    padding: u32,
    margin: u32,
    decoration: bool,
    shadow: bool,
) -> RgbaImage {
    let (layouts, total_content_height) =
        compute_layout(panels, show_cmd, font_config);

    let max_output_cols = panels.iter().map(|p| p.cols).max().unwrap_or(80);
    let max_header_cols = if show_cmd {
        panels
            .iter()
            .filter(|p| !p.command.is_empty())
            .map(|p| p.command.len() + 2) // "$ " prefix
            .max()
            .unwrap_or(0)
    } else {
        0
    };
    let max_cols = max_output_cols.max(max_header_cols);
    let content_width = (max_cols as f32 * font_config.cell_width) as u32;

    let title_bar_h = if decoration { theme.title_bar_height } else { 0 };

    let window_width = content_width + padding * 2;
    let window_height = total_content_height + padding * 2 + title_bar_h;

    let shadow_extra = if shadow { theme.shadow_offset } else { 0 };
    let img_width = window_width + margin * 2 + shadow_extra;
    let img_height = window_height + margin * 2 + shadow_extra;

    let outer_bg = if decoration {
        Rgba([0, 0, 0, 0])
    } else {
        theme.outer_bg_color
    };
    let mut img = RgbaImage::from_pixel(img_width, img_height, outer_bg);

    let win_x = margin as i32;
    let win_y = margin as i32;

    // Shadow
    if shadow {
        draw_shadow(
            &mut img,
            win_x,
            win_y,
            window_width,
            window_height,
            theme.shadow_offset,
            if decoration { theme.corner_radius } else { 0 },
            theme.shadow_color,
        );
    }

    let win_radius = if decoration { theme.corner_radius } else { 0 };

    // Window border (no-decoration mode only)
    if !decoration && theme.border_width > 0 {
        let bw = theme.border_width;
        draw_filled_rect_mut(
            &mut img,
            Rect::at(win_x - bw as i32, win_y - bw as i32)
                .of_size(window_width + bw * 2, window_height + bw * 2),
            theme.border_color,
        );
    }

    // Window background
    draw_rounded_rect_filled(
        &mut img,
        win_x,
        win_y,
        window_width,
        window_height,
        win_radius,
        theme.bg_color,
    );

    // Title bar
    if decoration {
        // Title bar background - fill top area
        draw_filled_rect_mut(
            &mut img,
            Rect::at(win_x, win_y + theme.corner_radius as i32)
                .of_size(window_width, title_bar_h - theme.corner_radius),
            theme.title_bar_color,
        );
        // Top part with rounded corners
        draw_rounded_rect_filled(
            &mut img,
            win_x,
            win_y,
            window_width,
            title_bar_h,
            theme.corner_radius,
            theme.title_bar_color,
        );

        // Divider line between title bar and content
        draw_filled_rect_mut(
            &mut img,
            Rect::at(win_x, win_y + title_bar_h as i32 - 1).of_size(window_width, 1),
            Rgba([30, 34, 42, 255]),
        );

        // Traffic light buttons
        let btn_y = win_y + theme.button_y_center;
        for (i, color) in [theme.close_color, theme.minimize_color, theme.maximize_color]
            .iter()
            .enumerate()
        {
            let btn_x = win_x + theme.button_x_start + (i as i32 * theme.button_spacing);
            draw_filled_circle_mut(&mut img, (btn_x, btn_y), theme.button_radius, *color);
        }
    }

    // Content area
    let content_x = win_x + padding as i32;
    let content_y = win_y + title_bar_h as i32 + padding as i32;

    for (panel_idx, panel) in panels.iter().enumerate() {
        let layout = &layouts[panel_idx];
        let panel_y = content_y + layout.y_offset as i32;

        let mut text_y = panel_y;

        // Command header
        if show_cmd && !panel.command.is_empty() {
            draw_text_mut(
                &mut img,
                theme.cmd_arrow_color,
                content_x,
                text_y,
                font_config.scale,
                &font_config.font,
                "\u{2192}",
            );
            let cmd_x = content_x + (2.0 * font_config.cell_width) as i32;
            draw_text_mut(
                &mut img,
                theme.default_fg,
                cmd_x,
                text_y,
                font_config.scale,
                &font_config.font,
                &panel.command,
            );
            text_y += layout.cmd_header_height as i32;
        }

        // Grid cells
        for (row_idx, row) in panel.grid.iter().enumerate() {
            if row_idx >= panel.rows {
                break;
            }
            for (col_idx, cell) in row.iter().enumerate() {
                let x = content_x + (col_idx as f32 * font_config.cell_width) as i32;
                let y = text_y + (row_idx as f32 * font_config.cell_height) as i32;

                let (mut fg_color, mut bg_color) = if cell.inverse {
                    (
                        resolve_color(&cell.bg, false, theme),
                        resolve_color(&cell.fg, true, theme),
                    )
                } else {
                    (
                        resolve_color(&cell.fg, true, theme),
                        resolve_color(&cell.bg, false, theme),
                    )
                };

                if cell.inverse
                    && matches!(cell.fg, CellColor::Default)
                    && matches!(cell.bg, CellColor::Default)
                {
                    std::mem::swap(&mut fg_color, &mut bg_color);
                }

                // Draw background if non-default
                if bg_color != theme.bg_color {
                    let cell_w = font_config.cell_width as u32;
                    let cell_h = font_config.cell_height as u32;
                    draw_filled_rect_mut(
                        &mut img,
                        Rect::at(x, y).of_size(cell_w, cell_h),
                        bg_color,
                    );
                }

                if cell.ch.trim().is_empty() {
                    continue;
                }

                let font = if cell.bold {
                    &font_config.bold_font
                } else {
                    &font_config.font
                };

                draw_text_mut(
                    &mut img,
                    fg_color,
                    x,
                    y,
                    font_config.scale,
                    font,
                    &cell.ch,
                );

                // Underline
                if cell.underline {
                    let underline_y = y + font_config.cell_height as i32 - 2;
                    let cell_w = font_config.cell_width as u32;
                    draw_filled_rect_mut(
                        &mut img,
                        Rect::at(x, underline_y).of_size(cell_w, 1),
                        fg_color,
                    );
                }
            }
        }
    }

    img
}
