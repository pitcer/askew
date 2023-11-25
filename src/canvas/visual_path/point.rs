use tiny_skia::{FillRule, Path, PathBuilder, PixmapMut, Point, Transform};

use crate::canvas::paint::PaintBuilder;
use crate::canvas::visual_path::private::VisualPathDetails;
use crate::canvas::visual_path::property::{AlphaProperty, ColorProperty, SizeProperty};
use crate::canvas::visual_path::VisualPath;
use crate::config::rgb::{Alpha, Rgb};

pub type VisualPoint = VisualPath<VisualPointDetails>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualPointProperties {
    radius: SizeProperty<f32>,
    color: ColorProperty,
    alpha: AlphaProperty,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualPointDetails;

impl VisualPathDetails for VisualPointDetails {
    type Properties = VisualPointProperties;

    fn draw_on(pixmap: &mut PixmapMut<'_>, path: &Path, properties: &Self::Properties) {
        let color = properties.color.value();
        let alpha = properties.alpha.value();
        let paint = PaintBuilder::new().rgba_color(color, alpha).build();
        pixmap.fill_path(path, &paint, FillRule::Winding, Transform::identity(), None);
    }

    fn build_path(
        mut builder: PathBuilder,
        points: impl ExactSizeIterator<Item = Point>,
        properties: &Self::Properties,
    ) -> Option<Path> {
        let radius = properties.radius.value();
        for point in points {
            builder.push_circle(point.x, point.y, radius);
        }

        builder.finish()
    }
}

impl VisualPointProperties {
    #[must_use]
    pub fn new(radius: f32, color: Rgb, alpha: Alpha) -> Self {
        let radius = SizeProperty::new(radius);
        let color = ColorProperty::new(color);
        let alpha = AlphaProperty::new(alpha);
        Self { radius, color, alpha }
    }
}
