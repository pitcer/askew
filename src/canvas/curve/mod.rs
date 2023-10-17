use crate::canvas::curve::control_points::ControlPointsCurveKind;
use crate::canvas::curve::event_handler::CurveEventHandler;
use crate::canvas::curve::formula::FormulaCurveKind;
use crate::canvas::v2::{DrawOn, Update};
use tiny_skia::PixmapMut;

pub mod control_points;
pub mod converter;
pub mod event_handler;
pub mod formula;
pub mod samples;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CurveKind {
    ControlPoints(ControlPointsCurveKind),
    Formula(FormulaCurveKind),
}

impl CurveKind {
    pub fn event_handler(&mut self) -> CurveEventHandler<'_> {
        CurveEventHandler::new(self)
    }
}

impl Update for CurveKind {
    fn update(&mut self) {
        match self {
            CurveKind::ControlPoints(curve) => curve.update(),
            CurveKind::Formula(curve) => curve.update(),
        }
    }
}

impl DrawOn for CurveKind {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        match self {
            CurveKind::ControlPoints(curve) => curve.draw_on(pixmap),
            CurveKind::Formula(curve) => curve.draw_on(pixmap),
        }
    }
}
