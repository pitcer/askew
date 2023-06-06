use crate::ui::color::{Alpha, Rgb};

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Pixel([u8; 4]);

impl Pixel {
    pub const fn from_rgba(rgb: Rgb, alpha: Alpha) -> Self {
        Self([rgb.blue(), rgb.green(), rgb.red(), alpha.alpha()])
    }

    pub fn blend(&mut self, foreground: Self) {
        for index in 0..3 {
            self.blend_primary(foreground, index);
        }
    }

    fn blend_primary(&mut self, Self(foreground): Self, index: usize) {
        let Self(pixel) = self;
        let alpha = foreground[3] as u32;
        pixel[index] = Self::mix(foreground[index] as u32, pixel[index] as u32, alpha)
    }

    fn mix(foreground: u32, background: u32, foreground_alpha: u32) -> u8 {
        let result = (foreground * foreground_alpha + background * (255 - foreground_alpha)) / 255;
        debug_assert!(result <= 255);
        result as u8
    }

    pub fn into_rgb_array(self) -> [u8; 3] {
        let Self([blue, green, red, _]) = self;
        [red, green, blue]
    }
}
