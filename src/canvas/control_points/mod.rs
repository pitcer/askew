use std::fmt::Debug;

use num_traits::{Float, Num, NumCast};

use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;

pub mod point;
pub mod request;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

    pub fn shift_all<V>(&mut self, vector: Vector<V>)
    where
        T: AsRef<Point<V>> + AsMut<Point<V>>,
        V: Copy + Num,
    {
        for point in &mut self.points {
            *point.as_mut() = *point.as_ref() + vector;
        }
    }

    pub fn shift<V>(&mut self, index: usize, vector: Vector<V>) -> Option<()>
    where
        T: AsRef<Point<V>> + AsMut<Point<V>>,
        V: Copy + Num,
    {
        if let Some(point) = self.points.get_mut(index) {
            *point.as_mut() = *point.as_ref() + vector;
            Some(())
        } else {
            None
        }
    }

    pub fn rotate_all<V>(&mut self, angle: V)
    where
        T: AsRef<Point<V>> + AsMut<Point<V>> + Debug + Copy + Into<Point<V>>,
        V: Float + Debug,
    {
        let cos_angle = V::cos(angle);
        let sin_angle = V::sin(angle);
        let Some(center) = self.center_of_mass() else {
            return;
        };
        for point in &mut self.points {
            let vector = *point.as_ref() - center;
            let rotated_horizontal =
                cos_angle * vector.horizontal() - sin_angle * vector.vertical();
            let rotated_vertical = sin_angle * vector.horizontal() + cos_angle * vector.vertical();
            let rotated = Vector::new(rotated_horizontal, rotated_vertical);
            *point.as_mut() = center + rotated;
        }
    }

    #[must_use]
    pub fn center_of_mass<V>(&self) -> Option<Point<V>>
    where
        T: Into<Point<V>> + Copy,
        V: Copy + Num + NumCast,
    {
        let length = V::from(self.points.len()).unwrap();
        self.points
            .iter()
            .copied()
            .map(|point| point.into().into_vector(Point::zero()))
            .reduce(|accumulator, point| accumulator + point)
            .map(|center| center / length)
            .map(|center| center.into_point(Point::zero()))
    }

    #[must_use]
    pub fn select_point(&self, guess: Point<f32>, radius: f32) -> Option<usize>
    where
        T: AsRef<Point<f32>> + Copy,
    {
        let radius_squared = radius * radius;
        self.points
            .iter()
            .enumerate()
            .find(|(_id, point)| guess.distance_squared(*point.as_ref()) <= radius_squared)
            .map(|(id, _)| id)
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.points
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
    pub fn copied_iterator(&self) -> impl ExactSizeIterator<Item = T> + '_
    where
        T: Copy,
    {
        self.points.iter().copied()
    }
}
