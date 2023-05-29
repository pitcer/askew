use tiny_skia::Path;

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
    algorithm: BezierAlgorithm,
}

#[derive(Debug)]
pub enum BezierAlgorithm {
    Generic,
    DeCasteljau,
    ChudyWozny,
}

impl Bezier {
    pub fn new(points: CurvePoints, samples: u32, algorithm: BezierAlgorithm) -> Self {
        Self {
            points,
            samples,
            algorithm,
        }
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

        let path = curve_path::equally_spaced(0.0..=1.0, self.samples as usize);
        match self.algorithm {
            BezierAlgorithm::Generic => {
                let path = path.map(|t| self.bezier(t));
                let path = CurvePath::from_iter(path);
                path.into_skia_path()
            }
            BezierAlgorithm::DeCasteljau => {
                let path = path.map(|t| math::de_casteljau(&self.points.points, t));
                let path = CurvePath::from_iter(path);
                path.into_skia_path()
            }
            BezierAlgorithm::ChudyWozny => {
                let path = path.map(|t| math::chudy_wozny(&self.points.points, t));
                let path = CurvePath::from_iter(path);
                path.into_skia_path()
            }
        }
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
