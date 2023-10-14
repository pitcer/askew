use anyhow::Result;
use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::curve::polyline::event_handler::PolylineCurveEventHandler;
use crate::canvas::v2::{DrawOn, Update};

pub mod event_handler;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolylineCurve {
    pub control_points: ControlPointsCurve<CurvePoint>,
    pub polyline: BasePolyline<false>,
}

impl PolylineCurve {
    #[must_use]
    pub fn new(
        control_points: ControlPointsCurve<CurvePoint>,
        polyline: BasePolyline<false>,
    ) -> Self {
        Self { control_points, polyline }
    }

    pub fn event_handler(&mut self) -> PolylineCurveEventHandler<'_> {
        PolylineCurveEventHandler::new(self)
    }
}

impl DrawOn for PolylineCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) -> Result<()> {
        if self.control_points.points.length() > 1 {
            self.polyline.draw_on(pixmap)?;
            self.control_points.draw_on(pixmap)?;
        }
        Ok(())
    }
}

impl Update for PolylineCurve {
    fn update(&mut self) -> Result<()> {
        if self.control_points.points.length() > 1 {
            let points = self.control_points.points.iterator().copied();
            self.polyline.rebuild_paths(points)?;
            self.control_points.rebuild_paths()?;
        }
        Ok(())
    }
}
