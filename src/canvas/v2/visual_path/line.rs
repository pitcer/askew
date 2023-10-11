use anyhow::anyhow;
use anyhow::Result;
use tiny_skia::{Path, PathBuilder, PixmapMut, Stroke, Transform};

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::paint::PaintBuilder;
use crate::canvas::v2::visual_path::private;
use crate::config::rgb::Rgb;

#[derive(Debug, Clone)]
pub struct VisualLine {
    path: Option<Path>,
    properties: VisualLineProperties,
}

#[derive(Debug, Copy, Clone)]
pub struct VisualLineProperties {
    closed: bool,
    visible: bool,
    width: f32,
    color: Rgb,
}

impl private::VisualPath for VisualLine {
    type Properties = VisualLineProperties;

    fn new_internal(path: Option<Path>, properties: Self::Properties) -> Self {
        Self { path, properties }
    }

    fn draw_on_internal(&self, mut pixmap: PixmapMut<'_>, path: &Path) -> Result<()> {
        let paint = PaintBuilder::new().rgb_color(self.properties.color).build();
        let stroke = Stroke { width: self.properties.width, ..Stroke::default() };
        pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
        Ok(())
    }

    fn build_path_from_builder<P>(
        mut builder: PathBuilder,
        mut points: impl Iterator<Item = P>,
        properties: &Self::Properties,
    ) -> Result<Path>
    where
        P: Into<CurvePoint>,
    {
        let point = points.next().ok_or_else(|| anyhow!("points should be non-empty"))?;
        let point = point.into();
        builder.move_to(point.horizontal(), point.vertical());

        for point in points {
            let point = point.into();
            builder.line_to(point.horizontal(), point.vertical());
        }

        if properties.closed {
            builder.close();
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

impl VisualLineProperties {
    #[must_use]
    pub fn new(closed: bool, visible: bool, width: f32, color: Rgb) -> Self {
        Self { closed, visible, width, color }
    }
}

impl private::VisualPathProperties for VisualLineProperties {
    fn visible(&self) -> bool {
        self.visible
    }
}
