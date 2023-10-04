use softbuffer::Buffer;
use tiny_skia::{FillRule, Paint, Path, PixmapMut, PixmapPaint, PixmapRef, Stroke, Transform};

use pixel::Pixel;

use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::math::size::Size;

pub mod bar;
pub mod pixel;

#[derive(Debug)]
pub struct Panel<'a> {
    buffer: &'a mut [Pixel],
    area: Rectangle<u32>,
}

impl<'a> Panel<'a> {
    pub fn new(buffer: &'a mut [Pixel], area: Rectangle<u32>) -> Self {
        Self { buffer, area }
    }

    pub fn from_buffer(buffer: &'a mut Buffer<'_>, area: Rectangle<u32>) -> Self {
        let buffer = bytemuck::cast_slice_mut(buffer);
        Self::new(buffer, area)
    }

    #[must_use]
    pub fn split_vertical<const HEIGHTS: usize>(
        self,
        heights: [usize; HEIGHTS],
    ) -> [Self; HEIGHTS] {
        let size = self.area.size();
        debug_assert_eq!(heights.into_iter().sum::<usize>(), size.height() as usize);

        let buffer_width = size.width();
        let mut data = self.buffer;
        let mut panels = [(); HEIGHTS].map(|_| None);

        for (height, panel) in heights.into_iter().zip(panels.iter_mut()) {
            let (split_data, remaining) = data.split_at_mut(buffer_width as usize * height);
            data = remaining;

            let area = Rectangle::new(self.area.origin(), Size::new(buffer_width, height as u32));
            let sub_panel = Self::new(split_data, area);
            panel.replace(sub_panel);
        }

        panels.map(|panel| panel.expect("panel should be initialized in the loop"))
    }

    pub fn fill(&mut self, pixel: Pixel) {
        self.buffer.fill(pixel);
    }

    pub fn draw_pixmap(&mut self, x: i32, y: i32, pixmap: PixmapRef<'_>) {
        self.as_pixmap_mut().draw_pixmap(
            x,
            y,
            pixmap,
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
    }

    pub fn draw_stroke_path(&mut self, path: &Path, paint: &Paint<'_>, stroke: &Stroke) {
        self.as_pixmap_mut().stroke_path(path, paint, stroke, Transform::identity(), None);
    }

    pub fn draw_fill_path(&mut self, path: &Path, paint: &Paint<'_>, fill_rule: FillRule) {
        self.as_pixmap_mut().fill_path(path, paint, fill_rule, Transform::identity(), None);
    }

    pub fn blend_pixel(&mut self, pixel: Point<usize>, foreground: Pixel) {
        let width = self.area.size().width() as usize;
        let index = pixel.horizontal() + pixel.vertical() * width;
        let pixel = &mut self.buffer[index];
        pixel.blend(foreground);
    }

    fn as_pixmap_mut(&mut self) -> PixmapMut<'_> {
        let bytes = bytemuck::cast_slice_mut(self.buffer);
        let size = self.area.size();
        PixmapMut::from_bytes(bytes, size.width(), size.height())
            .expect("panel area size is invalid")
    }

    #[must_use]
    pub fn area(&self) -> Rectangle<u32> {
        self.area
    }
}
