use tiny_skia::Path;

use crate::canvas::curve::control_points::{
    ControlPoints, CurvePoint, GetControlPoints, WeightedPoint,
};
use crate::canvas::curve::curve_path;
use crate::canvas::curve::curve_path::{CurvePath, ToPath};
use crate::canvas::math;
use crate::canvas::math::vector::Vector;
use crate::event::handler::{AddPointHandler, ChangePointWeightHandler, MovePointHandler};

#[derive(Debug)]
pub struct RationalBezier {
    points: RationalBezierPoints,
    samples: u32,
}

impl RationalBezier {
    pub fn new(points: RationalBezierPoints, samples: u32) -> Self {
        Self { points, samples }
    }

    fn rational_bezier(&self, t: f32) -> CurvePoint {
        let n = self.points.length() as u32 - 1;
        let result = self
            .points
            .iterator()
            .enumerate()
            .map(|(k, point)| {
                let bernstein = math::bernstein(n, k as u32, t);
                CurvePoint::new(
                    point.point.horizontal() * bernstein * point.weight,
                    point.point.vertical() * bernstein * point.weight,
                )
            })
            .reduce(|accumulator, point| {
                CurvePoint::new(
                    accumulator.horizontal() + point.horizontal(),
                    accumulator.vertical() + point.vertical(),
                )
            })
            .expect("points should not be empty");
        let divisor = self
            .points
            .iterator()
            .enumerate()
            .map(|(k, point)| point.weight * math::bernstein(n, k as u32, t))
            .sum::<f32>();
        CurvePoint::new(result.horizontal() / divisor, result.vertical() / divisor)
    }
}

pub type RationalBezierPoints = ControlPoints<RationalBezierPoint>;

pub type RationalBezierPoint = WeightedPoint<f32, f32>;

impl RationalBezierPoint {
    pub fn new(point: CurvePoint, weight: f32) -> Self {
        Self { point, weight }
    }
}

impl ToPath for RationalBezier {
    fn to_path(&self) -> Option<Path> {
        if self.points.length() < 2 {
            return None;
        }

        let path = curve_path::equally_spaced(0.0..=1.0, self.samples as usize)
            .map(|t| self.rational_bezier(t));
        let path = CurvePath::from_iter(path);
        path.into_skia_path()
    }
}

impl GetControlPoints for RationalBezier {
    type Point = RationalBezierPoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }

    fn control_points_mut(&mut self) -> &mut ControlPoints<Self::Point> {
        &mut self.points
    }
}

impl ChangePointWeightHandler for RationalBezier {
    fn handle_change_point_weight(
        &mut self,
        point_index: usize,
        weight_change: impl Fn(f32) -> f32,
    ) -> anyhow::Result<()> {
        self.points.map_weight(point_index, weight_change);
        Ok(())
    }
}

impl MovePointHandler for RationalBezier {
    fn handle_move_point(
        &mut self,
        point_index: usize,
        position_change: Vector<f32>,
    ) -> anyhow::Result<()> {
        self.control_points_mut()
            .shift(point_index, position_change);
        Ok(())
    }
}
