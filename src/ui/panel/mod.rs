use std::mem::MaybeUninit;

use tiny_skia::{
    FillRule, Paint, Path, Pixmap, PixmapMut, PixmapPaint, PixmapRef, Stroke, Transform,
};

use crate::canvas::math::rectangle::Rectangle;
use crate::ui::paint::BgraColor;

pub mod command;
pub mod status;

pub struct Panel {
    buffer: Pixmap,
    area: Rectangle<u32>,
}

impl Panel {
    pub fn new(buffer: Pixmap, area: Rectangle<u32>) -> Self {
        Self { buffer, area }
    }

    pub fn as_sub_panel(&mut self) -> SubPanel<'_> {
        let pixmap = self.buffer.as_mut();
        SubPanel::new(pixmap, self.area)
    }

    pub fn split_vertical<const LINES: usize>(
        &mut self,
        heights: [usize; LINES],
    ) -> [SubPanel<'_>; LINES] {
        debug_assert_eq!(
            heights.into_iter().sum::<usize>(),
            self.buffer.height() as usize
        );

        let buffer_width = self.buffer.width();
        let mut data = self.buffer.data_mut();

        let panels = MaybeUninit::<[MaybeUninit<SubPanel<'_>>; LINES]>::uninit();
        // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
        let mut panels = unsafe { panels.assume_init() };

        for (height, panel) in heights.into_iter().zip(panels.iter_mut()) {
            let (split_data, remaining) =
                data.split_at_mut(tiny_skia::BYTES_PER_PIXEL * buffer_width as usize * height);
            data = remaining;

            let pixmap = PixmapMut::from_bytes(split_data, buffer_width, height as u32).unwrap();
            let sub_panel = SubPanel::new(pixmap, self.area);
            panel.write(sub_panel);
        }

        // SAFETY: All panels were initialized in the previous loop.
        panels.map(|panel| unsafe { panel.assume_init() })
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

pub struct SubPanel<'a> {
    buffer: PixmapMut<'a>,
    area: Rectangle<u32>,
}

impl<'a> SubPanel<'a> {
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
