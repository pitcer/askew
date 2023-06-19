use crate::canvas::curve::control_points::ControlPointsCurveKind;
use crate::canvas::curve::event_handler::CurveEventHandler;
use crate::canvas::curve::formula::FormulaCurveKind;

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
