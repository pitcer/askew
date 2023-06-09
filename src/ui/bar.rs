use crate::ui::color::{Alpha, Rgb};
use crate::ui::font::{FontLayout, FontLoader, GlyphPixel, GlyphRaster, GlyphRasterizer};
use crate::ui::panel::Panel;
use crate::ui::pixel::Pixel;

pub struct TextPanel<'a> {
    panel: Panel<'a>,
    text_color: Rgb,
    background_color: Rgb,
}

impl<'a> TextPanel<'a> {
    #[must_use]
    pub fn new(panel: Panel<'a>, text_color: Rgb, background_color: Rgb) -> Self {
        Self {
            panel,
            text_color,
            background_color,
        }
    }

    pub fn fill(&mut self) {
        let pixel = Pixel::from_rgba(self.background_color, Alpha::max());
        self.panel.fill(pixel);
    }

    pub fn draw_layout(
        &mut self,
        font: &FontLoader,
        layout: &FontLayout,
        rasterizer: &mut GlyphRasterizer,
    ) {
        for position in layout.glyph_positions() {
            let glyph = rasterizer.rasterize(font, position);
            let color = position.user_data.unwrap_or(self.text_color);
            Self::draw_glyph(glyph, &mut self.panel, color);
        }
    }

    fn draw_glyph(glyph: GlyphRaster<'_>, panel: &mut Panel<'_>, color: Rgb) {
        for GlyphPixel { position, alpha } in glyph.iterator() {
            let pixel = Pixel::from_rgba(color, alpha);
            panel.blend_pixel(position, pixel);
        }
    }
}
