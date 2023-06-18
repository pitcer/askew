use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use num_traits::Num;

use crate::canvas::curve::control_points::WeightedPoint;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;

pub mod event_handler;

#[derive(Debug, Clone)]
pub struct ControlPoints<T> {
    points: Vec<T>,
}

impl<T> ControlPoints<T> {
    #[must_use]
    pub fn new(points: Vec<T>) -> Self {
        Self { points }
    }

    pub fn event_handler(&mut self) -> ControlPointsEventHandler<'_, T> {
        ControlPointsEventHandler::new(self)
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
    pub fn as_slice(&self) -> &[T] {
        &self.points
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
    pub fn shift(&mut self, index: usize, vector: Vector<T>) -> Option<()> {
        if let Some(point) = self.points.get_mut(index) {
            *point = *point + vector;
            Some(())
        } else {
            None
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
    pub fn shift(&mut self, index: usize, vector: Vector<T>) -> Option<()> {
        if let Some(point) = self.points.get_mut(index) {
            point.point = point.point + vector;
            Some(())
        } else {
            None
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
