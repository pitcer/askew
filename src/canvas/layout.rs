use anyhow::{anyhow, Result};
use tiny_skia::{Paint, Pixmap, PixmapMut, PixmapRef};
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
        self.buffer.fill(color.into())
    }

    pub fn buffer(&self) -> PixmapRef<'_> {
        self.buffer.as_ref()
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

    pub fn draw_path(&mut self, path: &Path, paint: &Paint<'_>, stroke: &Stroke) -> Result<()> {
        self.buffer
            .stroke_path(path, paint, stroke, Transform::identity(), None)
            .ok_or_else(|| anyhow!("Invalid arguments"))
    }

    pub fn area(&self) -> Rectangle<u32> {
        self.area
    }
}
