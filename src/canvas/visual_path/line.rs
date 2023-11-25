use tiny_skia::{Path, PathBuilder, PixmapMut, Point, Stroke, Transform};

use crate::canvas::paint::PaintBuilder;
use crate::canvas::visual_path::private::VisualPathDetails;
use crate::canvas::visual_path::property::{AlphaProperty, ColorProperty, SizeProperty};
use crate::canvas::visual_path::VisualPath;
use crate::config::rgb::{Alpha, Rgb};

pub type VisualLine<const CLOSED: bool> = VisualPath<VisualLineDetails<CLOSED>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualLineProperties {
    width: SizeProperty<f32>,
    color: ColorProperty,
    alpha: AlphaProperty,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualLineDetails<const CLOSED: bool>;

impl<const CLOSED: bool> VisualPathDetails for VisualLineDetails<CLOSED> {
    type Properties = VisualLineProperties;

    fn draw_on(pixmap: &mut PixmapMut<'_>, path: &Path, properties: &Self::Properties) {
        let color = properties.color.value();
        let alpha = properties.alpha.value();
        let paint = PaintBuilder::new().rgba_color(color, alpha).build();

        let width = properties.width.value();
        let stroke = Stroke { width, ..Stroke::default() };

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
    pub fn new(width: f32, color: Rgb, alpha: Alpha) -> Self {
        let width = SizeProperty::new(width);
        let color = ColorProperty::new(color);
        let alpha = AlphaProperty::new(alpha);
        Self { width, color, alpha }
    }
}
