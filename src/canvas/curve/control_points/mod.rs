use num_traits::Num;

use crate::canvas::curve::control_points::bezier::Bezier;
use crate::canvas::curve::control_points::interpolation::Interpolation;
use crate::canvas::curve::control_points::polyline::Polyline;
use crate::canvas::curve::control_points::rational_bezier::RationalBezier;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::event::handler::{AddPointHandler, DeletePointHandler, MovePointHandler};

pub mod bezier;
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
    Interpolation(Interpolation),
    Bezier(Bezier),
    RationalBezier(RationalBezier),
}

#[derive(Debug, Clone)]
pub struct ControlPoints<T> {
    points: Vec<T>,
}

impl<T> ControlPoints<T> {
    pub fn new(points: Vec<T>) -> Self {
        Self { points }
    }

    pub fn add(&mut self, point: T) {
        self.points.push(point)
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.points.len() {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.points.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.points.get_mut(index)
    }

    pub fn length(&self) -> usize {
        self.points.len()
    }

    pub fn iterator(&self) -> impl ExactSizeIterator<Item = &T> {
        self.points.iter()
    }

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
            *point = *point + vector
        }
    }

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
            point.point = point.point + vector
        }
    }

    pub fn map_weight(&mut self, index: usize, weight_change: impl Fn(W) -> W) {
        if let Some(point) = self.points.get_mut(index) {
            point.weight = weight_change(point.weight);
        }
    }

    pub fn points_iter<U>(&self) -> impl ExactSizeIterator<Item = Point<T>> + '_ {
        self.points.iter().map(|point| point.point)
    }
}

impl<T, U> AddPointHandler for T
where
    T: GetControlPoints<Point = U>,
    U: AsRef<Point<f32>>,
{
    type Point = U;

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