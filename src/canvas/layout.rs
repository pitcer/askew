use tiny_skia::{FillRule, Paint, Pixmap, PixmapMut, PixmapPaint, PixmapRef};
use tiny_skia_path::{Path, Stroke, Transform};

use crate::canvas::geometry::rectangle::Rectangle;
use crate::canvas::paint::BgraColor;

pub struct Layout {
    buffer: Pixmap,
    area: Rectangle<u32>,
}

impl Layout {
    pub fn new(buffer: Pixmap, area: Rectangle<u32>) -> Self {
        Self { buffer, area }
    }

    pub fn panel(&mut self) -> Panel<'_> {
        let pixmap = self.buffer.as_mut();
        Panel::new(pixmap, self.area)
    }

    pub fn fill(&mut self, color: BgraColor) {
        self.buffer.fill(color.into());
    }

    pub fn draw_pixmap(&mut self, x: i32, y: i32, pixmap: PixmapRef) {
        self.buffer.draw_pixmap(
            x,
            y,
            pixmap,
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        )
    }

    pub fn buffer(&self) -> PixmapRef<'_> {
        self.buffer.as_ref()
    }

    pub fn area(&self) -> Rectangle<u32> {
        self.area
    }
}

pub struct Panel<'a> {
    buffer: PixmapMut<'a>,
    area: Rectangle<u32>,
}

impl<'a> Panel<'a> {
    pub fn new(buffer: PixmapMut<'a>, area: Rectangle<u32>) -> Self {
        Self { buffer, area }
    }

    pub fn fill(&mut self, color: BgraColor) {
        self.buffer.fill(color.into())
    }

    pub fn draw_stroke_path(&mut self, path: &Path, paint: &Paint<'_>, stroke: &Stroke) {
        self.buffer
            .stroke_path(path, paint, stroke, Transform::identity(), None)
    }

    pub fn draw_fill_path(&mut self, path: &Path, paint: &Paint<'_>, fill_rule: FillRule) {
        self.buffer
            .fill_path(path, paint, fill_rule, Transform::identity(), None)
    }

    pub fn area(&self) -> Rectangle<u32> {
        self.area
    }
}
