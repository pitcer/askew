use tiny_skia::{Color, Paint};

use crate::config::rgb::{Alpha, Rgb};

#[derive(Debug, Default)]
pub struct PaintBuilder<'a> {
    paint: Paint<'a>,
}

impl<'a> PaintBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self { paint: Paint::default() }
    }

    #[must_use]
    pub fn rgb_color(self, color: Rgb) -> PaintBuilder<'a> {
        let color = PaintColor::from_rgb(color);
        self.color(color)
    }

    #[must_use]
    pub fn rgba_color(self, color: Rgb, alpha: Alpha) -> PaintBuilder<'a> {
        let color = PaintColor::from_rgba(color, alpha);
        self.color(color)
    }

    #[must_use]
    pub fn color(mut self, color: PaintColor) -> PaintBuilder<'a> {
        self.paint.set_color(color.0);
        self
    }

    #[must_use]
    pub fn build(self) -> Paint<'a> {
        self.paint
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PaintColor(Color);

impl PaintColor {
    #[must_use]
    pub fn from_rgb(rgb: Rgb) -> Self {
        Self::from_rgba(rgb, Alpha::OPAQUE)
    }

    #[must_use]
    pub fn from_rgba(rgb: Rgb, alpha: Alpha) -> Self {
        // Color format in display is 0RGB in [u8], but we store colors using u32, so after
        // bytemuck::cast bytes would get flipped, since we are on little endian.
        let color = Color::from_rgba8(rgb.blue(), rgb.green(), rgb.red(), alpha.alpha());
        Self(color)
    }
}
