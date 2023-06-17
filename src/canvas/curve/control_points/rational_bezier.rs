use anyhow::Result;

use crate::canvas::curve::control_points::{
    ControlPoints, CurvePoint, GetControlPoints, WeightedPoint,
};
use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};
use crate::canvas::math::vector::Vector;
use crate::canvas::{curve, math};
use crate::event::handler::{
    AddPointHandler, ChangePointWeightHandler, CurveEventError, MovePointHandler,
};

#[derive(Debug)]
pub struct RationalBezier {
    points: RationalBezierPoints,
    samples: u32,
    algorithm: RationalBezierAlgorithm,
}

#[derive(Debug)]
pub enum RationalBezierAlgorithm {
    Generic,
    ChudyWozny,
}

impl RationalBezier {
    #[must_use]
    pub fn new(
        points: RationalBezierPoints,
        samples: u32,
        algorithm: RationalBezierAlgorithm,
    ) -> Self {
        Self {
            points,
            samples,
            algorithm,
        }
    }

    pub fn samples_mut(&mut self) -> &mut u32 {
        &mut self.samples
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

impl ToPath for RationalBezier {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        if self.points.length() < 2 {
            return None;
        }

        let path = curve::equally_spaced(0.0..=1.0, self.samples as usize);
        match self.algorithm {
            RationalBezierAlgorithm::Generic => {
                let path = path.map(|t| self.rational_bezier(t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
            RationalBezierAlgorithm::ChudyWozny => {
                let path = path.map(|t| math::rational_chudy_wozny(&self.points.points, t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
        }
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
    ) -> Result<(), CurveEventError> {
        self.points.map_weight(point_index, weight_change);
        Ok(())
    }
}

impl MovePointHandler for RationalBezier {
    fn handle_move_point(
        &mut self,
        point_index: usize,
        position_change: Vector<f32>,
    ) -> Result<()> {
        self.control_points_mut()
            .shift(point_index, position_change);
        Ok(())
    }
}

impl AddPointHandler for RationalBezier {
    type Point = RationalBezierPoint;

    fn handle_add_point(&mut self, point: Self::Point) -> Result<()> {
        self.control_points_mut().add(point);
        Ok(())
    }
}
