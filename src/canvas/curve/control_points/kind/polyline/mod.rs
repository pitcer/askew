use crate::canvas::curve::control_points::kind::polyline::event_handler::PolylineEventHandler;
use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::{CurvePoint, CurvePoints, GetControlPoints};
use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};

pub mod event_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Polyline {
    points: CurvePoints,
}

impl Polyline {
    #[must_use]
    pub fn new(points: CurvePoints) -> Self {
        Self { points }
    }

    pub fn event_handler(&mut self) -> PolylineEventHandler<'_> {
        PolylineEventHandler::new(self)
    }
}

impl ToPath for Polyline {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        let path = self.points.iterator().copied();
        let path = CurvePath::new_open(path);
        converter.to_path(path)
    }
}

impl GetControlPoints for Polyline {
    type Point = CurvePoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }
}
