use anyhow::Result;
use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::v2::visual_path::line::{VisualLine, VisualLineProperties};
use crate::canvas::v2::visual_path::point::{VisualPoint, VisualPointProperties};
use crate::canvas::v2::DrawOn;
use crate::config::rgb::Rgb;
use crate::config::CanvasConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BasePolyline<const CLOSED: bool> {
    pub line: VisualLine<CLOSED>,
    pub points: VisualPoint,
}

impl<const CLOSED: bool> BasePolyline<CLOSED> {
    #[must_use]
    pub fn new(line: VisualLine<CLOSED>, points: VisualPoint) -> Self {
        Self { line, points }
    }

    #[must_use]
    pub fn from_config(config: &CanvasConfig) -> Self {
        Self {
            line: VisualLine::new(VisualLineProperties::new(
                true,
                config.default_line_width,
                config.line_color,
            )),
            points: VisualPoint::new(VisualPointProperties::new(false, 3.0, Rgb::WHITE)),
        }
    }

    pub fn rebuild_paths<P>(&mut self, points: impl ExactSizeIterator<Item = P>) -> Result<()>
    where
        P: Into<CurvePoint> + Copy,
    {
        let path = self.line.rebuild_path(points)?;
        either::for_both!(path, points => { let _ = self.points.rebuild_path(points)?; });
        Ok(())
    }
}

impl<const CLOSED: bool> DrawOn for BasePolyline<CLOSED> {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) -> Result<()> {
        self.line.draw_on(pixmap)?;
        self.points.draw_on(pixmap)?;
        Ok(())
    }
}

impl<const CLOSED: bool> Default for BasePolyline<CLOSED> {
    fn default() -> Self {
        Self {
            line: VisualLine::new(VisualLineProperties::new(true, 2.0, Rgb::WHITE)),
            points: VisualPoint::new(VisualPointProperties::new(false, 4.0, Rgb::WHITE)),
        }
    }
}
