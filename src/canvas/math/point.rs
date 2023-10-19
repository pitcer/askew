use std::num::NonZeroU32;
use std::ops::{Add, Sub};

use num_traits::Num;

use crate::canvas::math::vector::Vector;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Point<T> {
    horizontal: T,
    vertical: T,
}

impl<T> Point<T> {
    pub const fn new(horizontal: T, vertical: T) -> Self {
        Self { horizontal, vertical }
    }
}

impl<T> Point<T> {
    pub fn horizontal(self) -> T {
        self.horizontal
    }

    pub fn vertical(self) -> T {
        self.vertical
    }
}

impl<T> Point<T>
where
    T: Copy + Num,
{
    #[must_use]
    pub fn zero() -> Self {
        Point::new(T::zero(), T::zero())
    }

    pub fn distance_squared(self, other: Point<T>) -> T {
        let vector = self - other;
        vector.horizontal() * vector.horizontal() + vector.vertical() * vector.vertical()
    }

    pub fn into_vector(self, origin: Self) -> Vector<T> {
        self - origin
    }
}

impl<T> Add<Vector<T>> for Point<T>
where
    T: Copy + Num,
{
    type Output = Point<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point {
            horizontal: self.horizontal + rhs.horizontal(),
            vertical: self.vertical + rhs.vertical(),
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Copy + Num,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.horizontal - rhs.horizontal, self.vertical - rhs.vertical)
    }
}

impl From<Point<f32>> for tiny_skia::Point {
    fn from(value: Point<f32>) -> Self {
        Self { x: value.horizontal, y: value.vertical }
    }
}

impl From<tiny_skia::Point> for Point<f32> {
    fn from(value: tiny_skia::Point) -> Self {
        Self { horizontal: value.x, vertical: value.y }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(value: Point<T>) -> Self {
        (value.horizontal, value.vertical)
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Point { horizontal: value.0, vertical: value.1 }
    }
}

impl From<Point<NonZeroU32>> for Point<u32> {
    fn from(value: Point<NonZeroU32>) -> Self {
        Point { horizontal: value.horizontal.get(), vertical: value.vertical.get() }
    }
}

impl From<Point<u32>> for Point<f32> {
    fn from(value: Point<u32>) -> Self {
        Point { horizontal: value.horizontal as f32, vertical: value.vertical as f32 }
    }
}
