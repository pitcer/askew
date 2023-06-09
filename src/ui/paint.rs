use tiny_skia::{Color, Paint};

use crate::ui::color::{Alpha, Rgb};

#[derive(Debug, Default)]
pub struct PaintBuilder<'a> {
    paint: Paint<'a>,
}

impl<'a> PaintBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            paint: Paint::default(),
        }
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
        const ALPHA: Alpha = Alpha::max();
        Self::from_rgba(rgb, ALPHA)
    }

    #[must_use]
    pub fn from_rgba(rgb: Rgb, alpha: Alpha) -> Self {
        // Color format in display is 0RGB in [u8], but we store colors using u32, so after
        // bytemuck::cast bytes would get flipped, since we are on little endian.
        let color = Color::from_rgba8(rgb.blue(), rgb.green(), rgb.red(), alpha.alpha());
        Self(color)
    }
}
