use anyhow::anyhow;
use anyhow::Result;
use tiny_skia::{FillRule, Path, PathBuilder, PixmapMut, Transform};

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::paint::PaintBuilder;
use crate::canvas::v2::visual_path::private::{VisualPathDetails, VisualPathProperties};
use crate::canvas::v2::visual_path::VisualPath;
use crate::config::rgb::Rgb;

pub type VisualPoint = VisualPath<VisualPointDetails>;

#[derive(Debug, Copy, Clone)]
pub struct VisualPointProperties {
    visible: bool,
    radius: f32,
    color: Rgb,
}

#[derive(Debug, Clone)]
pub struct VisualPointDetails;

impl VisualPathDetails for VisualPointDetails {
    type Properties = VisualPointProperties;

    fn draw_on(
        mut pixmap: PixmapMut<'_>,
        path: &Path,
        properties: &Self::Properties,
    ) -> Result<()> {
        let paint = PaintBuilder::new().rgb_color(properties.color).build();
        pixmap.fill_path(path, &paint, FillRule::Winding, Transform::identity(), None);
        Ok(())
    }

    fn build_path_from_builder<P>(
        mut builder: PathBuilder,
        points: impl Iterator<Item = P>,
        properties: &Self::Properties,
    ) -> Result<Path>
    where
        P: Into<CurvePoint>,
    {
        for point in points {
            let point = point.into();
            builder.push_circle(point.horizontal(), point.vertical(), properties.radius);
        }

        builder.finish().ok_or_else(|| anyhow!("path should be non-empty and have valid bounds"))
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
