use anyhow::anyhow;
use anyhow::Result;
use tiny_skia::{Path, PathBuilder, PixmapMut, Stroke, Transform};

use crate::canvas::curve::control_points::CurvePoint;
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

    fn draw_on(
        pixmap: &mut PixmapMut<'_>,
        path: &Path,
        properties: &Self::Properties,
    ) -> Result<()> {
        let paint = PaintBuilder::new().rgb_color(properties.color).build();
        let stroke = Stroke { width: properties.width, ..Stroke::default() };
        pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
        Ok(())
    }

    fn build_path_from_builder<P>(
        mut builder: PathBuilder,
        mut points: impl Iterator<Item = P>,
        _properties: &Self::Properties,
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

        if CLOSED {
            builder.close();
        }

        builder.finish().ok_or_else(|| anyhow!("path should be non-empty and have valid bounds"))
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
