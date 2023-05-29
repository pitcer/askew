use std::num::NonZeroU32;

use tiny_skia_path::IntSize;
use winit::dpi::PhysicalSize;

#[derive(Debug, Copy, Clone)]
pub struct Size<T> {
    width: T,
    height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T> Size<T>
where
    T: Copy,
{
    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }
}

impl From<Size<NonZeroU32>> for Size<u32> {
    fn from(value: Size<NonZeroU32>) -> Self {
        Size {
            width: value.width.get(),
            height: value.height.get(),
        }
    }
}

impl From<IntSize> for Size<NonZeroU32> {
    fn from(value: IntSize) -> Self {
        let width = NonZeroU32::new(value.width()).expect("IntSize::width should be non zero");
        let height = NonZeroU32::new(value.height()).expect("IntSize::height should be non zero");
        Size { width, height }
    }
}

impl<T> From<PhysicalSize<T>> for Size<T>
where
    T: Copy,
{
    fn from(value: PhysicalSize<T>) -> Self {
        Size {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<Size<u32>> for Size<f32> {
    fn from(value: Size<u32>) -> Self {
        Size {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}
