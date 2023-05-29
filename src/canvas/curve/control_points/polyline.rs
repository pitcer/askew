use tiny_skia::Path;

use crate::canvas::curve::control_points::{
    ControlPoints, CurvePoint, CurvePoints, GetControlPoints,
};
use crate::canvas::curve::curve_path::{CurvePath, ToPath};

#[derive(Debug)]
pub struct Polyline {
    points: CurvePoints,
}

impl Polyline {
    pub fn new(points: CurvePoints) -> Self {
        Self { points }
    }
}

impl ToPath for Polyline {
    fn to_path(&self) -> Option<Path> {
        let path = self.points.iterator().copied();
        let path = CurvePath::new(path);
        path.into_skia_path()
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
