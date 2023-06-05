use tiny_skia::{Color, Paint};

use crate::ui::color::Rgb;

#[derive(Debug, Default)]
pub struct PaintBuilder<'a> {
    paint: Paint<'a>,
}

impl<'a> PaintBuilder<'a> {
    pub fn new() -> Self {
        Self {
            paint: Paint::default(),
        }
    }

    pub fn color(mut self, color: PaintColor) -> PaintBuilder<'a> {
        self.paint.set_color(color.0);
        self
    }

    pub fn build(self) -> Paint<'a> {
        self.paint
    }
}

pub struct PaintColor(Color);

impl PaintColor {
    pub fn from_rgba(rgb: Rgb, alpha: u8) -> Self {
        // Color format in display is 0RGB in [u8], but we store colors using u32, so after
        // bytemuck::cast bytes would get flipped, since we are on little endian.
        let color = Color::from_rgba8(rgb.blue(), rgb.green(), rgb.red(), alpha);
        Self(color)
    }
}
