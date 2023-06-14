use crate::canvas::curve::control_points::{
    ControlPoints, CurvePoint, CurvePoints, GetControlPoints,
};
use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};

#[derive(Debug)]
pub struct Polyline {
    points: CurvePoints,
}

impl Polyline {
    #[must_use]
    pub fn new(points: CurvePoints) -> Self {
        Self { points }
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

    fn control_points_mut(&mut self) -> &mut ControlPoints<Self::Point> {
        &mut self.points
    }
}
