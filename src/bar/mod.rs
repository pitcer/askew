use anyhow::Result;

use crate::bar::color::Rgb;
use crate::bar::font::{FontLayout, FontLoader, GlyphRasterizer};
use crate::canvas::math::point::Point;
use crate::ui::paint::PaintColor;
use crate::ui::panel::SubPanel;

pub mod color;
pub mod command;
pub mod font;
pub mod status;

const BACKGROUND_COLOR: Rgb = Rgb::new(32, 32, 32);
const FONT_COLOR: Rgb = Rgb::new(249, 250, 244);

pub struct Bar {}

impl Bar {
    pub fn new(mut panel: SubPanel<'_>, text: &str) -> Result<Self> {
        panel.fill(PaintColor::from_rgba(
            BACKGROUND_COLOR.red(),
            BACKGROUND_COLOR.green(),
            BACKGROUND_COLOR.blue(),
            255,
        ));
        let font = FontLoader::new("JetBrainsMonoNL-Regular.ttf")?;
        let mut rasterizer = GlyphRasterizer::new(&font);
        let mut layout = FontLayout::new(&font, 16);
        layout.configure().with_text(text);

        for glyph in layout.glyphs() {
            let (metrics, raster) = rasterizer.rasterize(glyph);

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let alpha = raster[x + y * metrics.width];
                    panel.blend_pixel(
                        Point::new(glyph.x as usize + x, glyph.y as usize + y),
                        FONT_COLOR.with_alpha(alpha),
                    );
                }
            }
        }
        Ok(Self {})
    }
}
