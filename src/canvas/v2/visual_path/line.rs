use tiny_skia::{Path, PathBuilder, PixmapMut, Point, Stroke, Transform};

use crate::canvas::paint::PaintBuilder;
use crate::canvas::v2::visual_path::private::{VisualPathDetails, VisualPathProperties};
use crate::canvas::v2::visual_path::VisualPath;
use crate::config::rgb::Rgb;

pub type VisualLine<const CLOSED: bool> = VisualPath<VisualLineDetails<CLOSED>>;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualLineProperties {
    visible: bool,
    width: f32,
    color: Rgb,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualLineDetails<const CLOSED: bool>;

impl<const CLOSED: bool> VisualPathDetails for VisualLineDetails<CLOSED> {
    type Properties = VisualLineProperties;

    fn draw_on(pixmap: &mut PixmapMut<'_>, path: &Path, properties: &Self::Properties) {
        let paint = PaintBuilder::new().rgb_color(properties.color).build();
        let stroke = Stroke { width: properties.width, ..Stroke::default() };
        pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
    }

    fn build_path(
        mut builder: PathBuilder,
        mut points: impl ExactSizeIterator<Item = Point>,
        _properties: &Self::Properties,
    ) -> Option<Path> {
        let point = points.next()?;
        builder.move_to(point.x, point.y);

        for point in points {
            builder.line_to(point.x, point.y);
        }

        if CLOSED {
            builder.close();
        }

        builder.finish()
    }
}

impl VisualLineProperties {
    #[must_use]
    pub fn new(visible: bool, width: f32, color: Rgb) -> Self {
        Self { visible, width, color }
    }
}

impl VisualPathProperties for VisualLineProperties {
    fn visible(&self) -> bool {
        self.visible
    }
}
