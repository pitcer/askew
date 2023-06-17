use std::fmt::{Display, Formatter};

use num_traits::Num;

use crate::canvas::curve::control_points::bezier::Bezier;
use crate::canvas::curve::control_points::convex_hull::ConvexHull;
use crate::canvas::curve::control_points::event_handler::CurveEventHandler;
use crate::canvas::curve::control_points::interpolation::Interpolation;
use crate::canvas::curve::control_points::polyline::Polyline;
use crate::canvas::curve::control_points::rational_bezier::RationalBezier;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::event::handler::{
    AddPointHandler, ChangePointWeightHandler, CurveEventError, DeletePointHandler,
    MovePointHandler,
};

pub mod bezier;
pub mod convex_hull;
pub mod event_handler;
pub mod interpolation;
pub mod polyline;
pub mod rational_bezier;

pub type CurvePoints = ControlPoints<CurvePoint>;

pub type CurvePoint = Point<f32>;

pub trait GetControlPoints {
    type Point: AsRef<CurvePoint>;

    fn control_points(&self) -> &ControlPoints<Self::Point>;

    fn control_points_mut(&mut self) -> &mut ControlPoints<Self::Point>;
}

#[derive(Debug)]
pub enum ControlPointsCurve {
    Polyline(Polyline),
    ConvexHull(ConvexHull),
    Interpolation(Interpolation),
    Bezier(Bezier),
    RationalBezier(RationalBezier),
}

impl ControlPointsCurve {
    pub fn event_handler(&mut self) -> CurveEventHandler<'_> {
        CurveEventHandler::new(self)
    }

    pub fn samples_mut(&mut self) -> Option<&mut u32> {
        match self {
            ControlPointsCurve::Polyline(_) | ControlPointsCurve::ConvexHull(_) => None,
            ControlPointsCurve::Interpolation(curve) => Some(curve.samples_mut()),
            ControlPointsCurve::Bezier(curve) => Some(curve.samples_mut()),
            ControlPointsCurve::RationalBezier(curve) => Some(curve.samples_mut()),
        }
    }
}

impl Display for ControlPointsCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlPointsCurve::Polyline(_) => write!(f, "polyline"),
            ControlPointsCurve::Interpolation(_) => write!(f, "interpolation"),
            ControlPointsCurve::Bezier(_) => write!(f, "bezier"),
            ControlPointsCurve::RationalBezier(_) => write!(f, "rational_bezier"),
            ControlPointsCurve::ConvexHull(_) => write!(f, "convex_hull"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ControlPoints<T> {
    points: Vec<T>,
}

impl<T> ControlPoints<T> {
    #[must_use]
    pub fn new(points: Vec<T>) -> Self {
        Self { points }
    }

    pub fn add(&mut self, point: T) {
        self.points.push(point);
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.points.len() {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.points.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.points.get_mut(index)
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.points.len()
    }

    #[must_use]
    pub fn iterator(&self) -> impl ExactSizeIterator<Item = &T> {
        self.points.iter()
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.points
    }
}

impl<T> ControlPoints<Point<T>>
where
    T: Copy + Num,
{
    pub fn shift(&mut self, index: usize, vector: Vector<T>) {
        if let Some(point) = self.points.get_mut(index) {
            *point = *point + vector;
        }
    }

    #[must_use]
    pub fn points_iter(&self) -> impl ExactSizeIterator<Item = &Point<T>> {
        self.points.iter()
    }
}

impl<T, W> ControlPoints<WeightedPoint<T, W>>
where
    T: Copy + Num,
    W: Copy,
{
    pub fn shift(&mut self, index: usize, vector: Vector<T>) {
        if let Some(point) = self.points.get_mut(index) {
            point.point = point.point + vector;
        }
    }

    pub fn map_weight(&mut self, index: usize, weight_change: impl Fn(W) -> W) {
        if let Some(point) = self.points.get_mut(index) {
            point.weight = weight_change(point.weight);
        }
    }

    pub fn point_mut(&mut self, index: usize) -> Option<&mut WeightedPoint<T, W>> {
        self.points.get_mut(index)
    }

    #[must_use]
    pub fn points_iter(&self) -> impl ExactSizeIterator<Item = Point<T>> + '_ {
        self.points.iter().map(|point| point.point)
    }
}

impl<T> AddPointHandler for T
where
    T: GetControlPoints<Point = Point<f32>>,
    // U: AsRef<Point<f32>>,
{
    type Point = Point<f32>;

    fn handle_add_point(&mut self, point: Self::Point) -> anyhow::Result<()> {
        self.control_points_mut().add(point);
        Ok(())
    }
}

impl<T> MovePointHandler for T
where
    T: GetControlPoints<Point = Point<f32>>,
{
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

impl<T> DeletePointHandler for T
where
    T: GetControlPoints,
{
    fn handle_delete_point(&mut self, point_index: usize) -> anyhow::Result<()> {
        self.control_points_mut().remove(point_index);
        Ok(())
    }
}

impl<T> ChangePointWeightHandler for T
where
    T: GetControlPoints<Point = Point<f32>>,
{
    fn handle_change_point_weight(
        &mut self,
        _point_index: usize,
        _weight_change: impl Fn(f32) -> f32,
    ) -> Result<(), CurveEventError> {
        Err(CurveEventError::Unimplemented)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WeightedPoint<T, W> {
    point: Point<T>,
    weight: W,
}

impl<T, W> WeightedPoint<T, W> {
    pub fn new(point: Point<T>, weight: W) -> Self {
        Self { point, weight }
    }

    pub fn point(self) -> Point<T> {
        self.point
    }

    pub fn weight(self) -> W {
        self.weight
    }
}

impl<T, W> AsRef<Point<T>> for WeightedPoint<T, W> {
    fn as_ref(&self) -> &Point<T> {
        &self.point
    }
}

impl<T> AsRef<Point<T>> for Point<T> {
    fn as_ref(&self) -> &Point<T> {
        self
    }
}
