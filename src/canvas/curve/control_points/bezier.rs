use tiny_skia_path::Path;

use crate::canvas::curve::control_points::{
    ControlPoints, CurvePoint, CurvePoints, GetControlPoints,
};
use crate::canvas::curve::curve_path;
use crate::canvas::curve::curve_path::{CurvePath, ToPath};
use crate::canvas::math;

#[derive(Debug)]
pub struct Bezier {
    points: CurvePoints,
    samples: u32,
}

impl Bezier {
    pub fn new(points: CurvePoints, samples: u32) -> Self {
        Self { points, samples }
    }

    fn bezier(&self, t: f32) -> CurvePoint {
        let n = self.points.length() as u32 - 1;
        self.points
            .iterator()
            .enumerate()
            .map(|(k, point)| {
                let bernstein = math::bernstein(n, k as u32, t);
                CurvePoint::new(point.horizontal() * bernstein, point.vertical() * bernstein)
            })
            .reduce(|accumulator, point| {
                CurvePoint::new(
                    accumulator.horizontal() + point.horizontal(),
                    accumulator.vertical() + point.vertical(),
                )
            })
            .expect("points should not be empty")
    }
}

impl ToPath for Bezier {
    fn to_path(&self) -> Option<Path> {
        if self.points.length() < 2 {
            return None;
        }

        let path =
            curve_path::equally_spaced(0.0..=1.0, self.samples as usize).map(|t| self.bezier(t));
        let path = CurvePath::from_iter(path);
        path.into_skia_path()
    }
}

impl GetControlPoints for Bezier {
    type Point = CurvePoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }

    fn control_points_mut(&mut self) -> &mut ControlPoints<Self::Point> {
        &mut self.points
    }
}
