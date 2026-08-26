use image::Rgba;

use crate::parser::CellColor;

pub struct Theme {
    pub bg_color: Rgba<u8>,
    pub title_bar_color: Rgba<u8>,
    pub title_bar_height: u32,
    pub button_radius: i32,
    pub button_y_center: i32,
    pub button_x_start: i32,
    pub button_spacing: i32,
    pub close_color: Rgba<u8>,
    pub minimize_color: Rgba<u8>,
    pub maximize_color: Rgba<u8>,
    pub corner_radius: u32,
    pub shadow_color: Rgba<u8>,
    pub shadow_offset: u32,
    pub default_fg: Rgba<u8>,
    #[allow(dead_code)]
    pub divider_color: Rgba<u8>,
    #[allow(dead_code)]
    pub divider_height: u32,
    pub cmd_header_color: Rgba<u8>,
    pub cmd_arrow_color: Rgba<u8>,
    pub outer_bg_color: Rgba<u8>,
    pub border_color: Rgba<u8>,
    pub border_width: u32,
}

impl Theme {
    pub fn scaled(&self, s: u32) -> Theme {
        let si = s as i32;
        Theme {
            bg_color: self.bg_color,
            title_bar_color: self.title_bar_color,
            title_bar_height: self.title_bar_height * s,
            button_radius: self.button_radius * si,
            button_y_center: self.button_y_center * si,
            button_x_start: self.button_x_start * si,
            button_spacing: self.button_spacing * si,
            close_color: self.close_color,
            minimize_color: self.minimize_color,
            maximize_color: self.maximize_color,
            corner_radius: self.corner_radius * s,
            shadow_color: self.shadow_color,
            shadow_offset: self.shadow_offset * s,
            default_fg: self.default_fg,
            divider_color: self.divider_color,
            divider_height: self.divider_height * s,
            cmd_header_color: self.cmd_header_color,
            cmd_arrow_color: self.cmd_arrow_color,
            outer_bg_color: self.outer_bg_color,
            border_color: self.border_color,
            border_width: self.border_width * s,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg_color: Rgba([21, 21, 21, 255]),
            title_bar_color: Rgba([50, 54, 62, 255]),
            title_bar_height: 38,
            button_radius: 7,
            button_y_center: 19,
            button_x_start: 20,
            button_spacing: 22,
            close_color: Rgba([255, 95, 87, 255]),
            minimize_color: Rgba([255, 189, 46, 255]),
            maximize_color: Rgba([39, 201, 63, 255]),
            corner_radius: 10,
            shadow_color: Rgba([0, 0, 0, 80]),
            shadow_offset: 8,
            default_fg: Rgba([211, 211, 211, 255]),
            divider_color: Rgba([60, 64, 72, 255]),
            divider_height: 2,
            cmd_header_color: Rgba([128, 128, 128, 255]),
            cmd_arrow_color: Rgba([0, 255, 0, 255]),
            outer_bg_color: Rgba([30, 30, 30, 255]),
            border_color: Rgba([80, 84, 92, 255]),
            border_width: 1,
        }
    }
}

const ANSI_STANDARD: [[u8; 3]; 16] = [
    [0, 0, 0],       // 0  black
    [205, 49, 49],    // 1  red
    [13, 188, 121],   // 2  green
    [229, 229, 16],   // 3  yellow
    [36, 114, 200],   // 4  blue
    [188, 63, 188],   // 5  magenta
    [17, 168, 205],   // 6  cyan
    [229, 229, 229],  // 7  white
    [102, 102, 102],  // 8  bright black
    [241, 76, 76],    // 9  bright red
    [35, 209, 139],   // 10 bright green
    [245, 245, 67],   // 11 bright yellow
    [59, 142, 234],   // 12 bright blue
    [214, 112, 214],  // 13 bright magenta
    [41, 184, 219],   // 14 bright cyan
    [255, 255, 255],  // 15 bright white
];

pub fn ansi_index_to_rgba(idx: u8) -> Rgba<u8> {
    if idx < 16 {
        let c = ANSI_STANDARD[idx as usize];
        return Rgba([c[0], c[1], c[2], 255]);
    }
    if idx < 232 {
        let idx = idx - 16;
        let r = (idx / 36) * 51;
        let g = ((idx % 36) / 6) * 51;
        let b = (idx % 6) * 51;
        return Rgba([r, g, b, 255]);
    }
    let gray = 8 + (idx - 232) * 10;
    Rgba([gray, gray, gray, 255])
}

pub fn resolve_color(color: &CellColor, is_fg: bool, theme: &Theme) -> Rgba<u8> {
    match color {
        CellColor::Default => {
            if is_fg {
                theme.default_fg
            } else {
                theme.bg_color
            }
        }
        CellColor::Indexed(idx) => ansi_index_to_rgba(*idx),
        CellColor::Rgb(r, g, b) => Rgba([*r, *g, *b, 255]),
    }
}
