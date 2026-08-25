use ab_glyph::{Font, FontArc, PxScale, ScaleFont};

const FONT_REGULAR: &[u8] = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
const FONT_BOLD: &[u8] = include_bytes!("../assets/JetBrainsMono-Bold.ttf");

pub struct FontConfig {
    pub font: FontArc,
    pub bold_font: FontArc,
    pub cell_width: f32,
    pub cell_height: f32,
    pub scale: PxScale,
}

pub fn load_fonts(size: f32) -> FontConfig {
    let font = FontArc::try_from_slice(FONT_REGULAR).expect("failed to load regular font");
    let bold_font = FontArc::try_from_slice(FONT_BOLD).expect("failed to load bold font");
    let scale = PxScale::from(size);
    let (cw, ch) = measure_cell(&font, scale);
    FontConfig {
        font,
        bold_font,
        cell_width: cw,
        cell_height: ch,
        scale,
    }
}

pub fn measure_cell(font: &FontArc, scale: PxScale) -> (f32, f32) {
    let scaled = font.as_scaled(scale);
    let advance = scaled.h_advance(scaled.glyph_id('M'));
    let height = scaled.height();
    (advance, height * 1.2)
}

pub fn compute_font_size(
    total_content_rows: usize,
    max_content_cols: usize,
    padding: u32,
    margin: u32,
    title_bar_height: u32,
    max_image_width: u32,
    max_image_height: u32,
) -> f32 {
    let base_size: f32 = 16.0;
    let min_size: f32 = 8.0;

    if total_content_rows == 0 || max_content_cols == 0 {
        return base_size;
    }

    let chrome_x = (padding + margin) as f32 * 2.0;
    let chrome_y = (padding + margin) as f32 * 2.0 + title_bar_height as f32;

    let char_aspect = 0.6;
    let size_by_width = (max_image_width as f32 - chrome_x) / (max_content_cols as f32 * char_aspect);

    let line_height_factor = 1.2;
    let size_by_height =
        (max_image_height as f32 - chrome_y) / (total_content_rows as f32 * line_height_factor);

    base_size.min(size_by_width).min(size_by_height).max(min_size)
}
