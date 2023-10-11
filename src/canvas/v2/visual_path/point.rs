use anyhow::anyhow;
use anyhow::Result;
use tiny_skia::{FillRule, Path, PathBuilder, PixmapMut, Transform};

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::paint::PaintBuilder;
use crate::canvas::v2::visual_path::private;
use crate::config::rgb::Rgb;

#[derive(Debug, Clone)]
pub struct VisualPoint {
    path: Option<Path>,
    properties: VisualPointProperties,
}

#[derive(Debug, Copy, Clone)]
pub struct VisualPointProperties {
    visible: bool,
    radius: f32,
    color: Rgb,
}

impl private::VisualPath for VisualPoint {
    type Properties = VisualPointProperties;

    fn new_internal(path: Option<Path>, properties: Self::Properties) -> Self {
        Self { path, properties }
    }

    fn draw_on_internal(&self, mut pixmap: PixmapMut<'_>, path: &Path) -> Result<()> {
        let paint = PaintBuilder::new().rgb_color(self.properties.color).build();
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

    fn properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn path(&self) -> &Option<Path> {
        &self.path
    }

    fn path_mut(&mut self) -> &mut Option<Path> {
        &mut self.path
    }
}

impl VisualPointProperties {
    #[must_use]
    pub fn new(visible: bool, radius: f32, color: Rgb) -> Self {
        Self { visible, radius, color }
    }
}

impl private::VisualPathProperties for VisualPointProperties {
    fn visible(&self) -> bool {
        self.visible
    }
}
