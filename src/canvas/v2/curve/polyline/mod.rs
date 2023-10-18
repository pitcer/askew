use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::v2::base_polyline::VisualBaseLine;
use crate::canvas::v2::control_points_curve::VisualControlPoints;
use crate::canvas::v2::curve::polyline::event_handler::{
    PolylineCurveEventHandler, PolylineCurveEventHandlerMut,
};
use crate::canvas::v2::{DrawOn, Update};

pub mod event_handler;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolylineCurve {
    pub points: ControlPoints<CurvePoint>,
    pub control_points: VisualControlPoints,
    pub polyline: VisualBaseLine<false>,
}

impl PolylineCurve {
    #[must_use]
    pub fn new(
        points: ControlPoints<CurvePoint>,
        control_points: VisualControlPoints,
        polyline: VisualBaseLine<false>,
    ) -> Self {
        Self { points, control_points, polyline }
    }

    #[must_use]
    pub fn event_handler(&self) -> PolylineCurveEventHandler<'_> {
        PolylineCurveEventHandler::new(self)
    }

    pub fn event_handler_mut(&mut self) -> PolylineCurveEventHandlerMut<'_> {
        PolylineCurveEventHandlerMut::new(self)
    }
}

impl Update for PolylineCurve {
    fn update(&mut self) {
        let points = self.points.copied_iterator();
        self.polyline.rebuild_paths(points);

        self.control_points.rebuild_paths(&self.points);
    }
}

impl DrawOn for PolylineCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.polyline.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}
