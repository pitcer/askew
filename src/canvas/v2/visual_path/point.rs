use tiny_skia::{FillRule, Path, PathBuilder, PixmapMut, Point, Transform};

use crate::canvas::paint::PaintBuilder;
use crate::canvas::v2::visual_path::private::{VisualPathDetails, VisualPathProperties};
use crate::canvas::v2::visual_path::VisualPath;
use crate::config::rgb::Rgb;

pub type VisualPoint = VisualPath<VisualPointDetails>;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualPointProperties {
    visible: bool,
    radius: f32,
    color: Rgb,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualPointDetails;

impl VisualPathDetails for VisualPointDetails {
    type Properties = VisualPointProperties;

    fn draw_on(pixmap: &mut PixmapMut<'_>, path: &Path, properties: &Self::Properties) {
        let paint = PaintBuilder::new().rgb_color(properties.color).build();
        pixmap.fill_path(path, &paint, FillRule::Winding, Transform::identity(), None);
    }

    fn build_path(
        mut builder: PathBuilder,
        points: impl ExactSizeIterator<Item = Point>,
        properties: &Self::Properties,
    ) -> Option<Path> {
        for point in points {
            builder.push_circle(point.x, point.y, properties.radius);
        }

        builder.finish()
    }
}

impl VisualPointProperties {
    #[must_use]
    pub fn new(visible: bool, radius: f32, color: Rgb) -> Self {
        Self { visible, radius, color }
    }
}

impl VisualPathProperties for VisualPointProperties {
    fn visible(&self) -> bool {
        self.visible
    }
}
