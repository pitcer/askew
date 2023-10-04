use std::ops::{Add, Div, Mul, Sub};

use num_traits::Num;

use crate::canvas::math::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Vector<T> {
    horizontal: T,
    vertical: T,
}

impl<T> Vector<T> {
    pub fn new(horizontal: T, vertical: T) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

impl<T> Vector<T> {
    pub fn horizontal(self) -> T {
        self.horizontal
    }

    pub fn vertical(self) -> T {
        self.vertical
    }
}

impl<T> Vector<T>
where
    T: Copy + Num,
{
    pub fn cross_product_magnitude(&self, other: Self) -> T {
        self.horizontal * other.vertical - self.vertical * other.horizontal
    }

    pub fn into_point(self, origin: Point<T>) -> Point<T> {
        origin + self
    }
}

impl<T> Add<Vector<T>> for Vector<T>
where
    T: Copy + Num,
{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self {
            horizontal: self.horizontal + rhs.horizontal,
            vertical: self.vertical + rhs.vertical,
        }
    }
}

impl<T> Sub<Vector<T>> for Vector<T>
where
    T: Copy + Num,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self {
            horizontal: self.horizontal - rhs.horizontal,
            vertical: self.vertical - rhs.vertical,
        }
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Copy + Num,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            horizontal: self.horizontal * rhs,
            vertical: self.vertical * rhs,
        }
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Copy + Num,
{
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            horizontal: self.horizontal / rhs,
            vertical: self.vertical / rhs,
        }
    }
}
