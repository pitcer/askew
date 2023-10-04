use std::num::NonZeroU32;

use num_traits::Num;

use crate::canvas::math::point::Point;
use crate::canvas::math::size::Size;

/// Rectangle represented by origin (left bottom point) and its size.
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rectangle<T> {
    origin: Point<T>,
    size: Size<T>,
}

impl<T> Rectangle<T> {
    pub fn new(origin: Point<T>, size: Size<T>) -> Self {
        Self { origin, size }
    }
}

impl<T> Rectangle<T>
where
    T: Copy,
{
    pub fn origin(&self) -> Point<T> {
        self.origin
    }

    pub fn size(&self) -> Size<T> {
        self.size
    }
}

impl<T> Rectangle<T>
where
    T: Copy + Num,
{
    pub fn sides_ratio(&self) -> T {
        self.size.width() / self.size.height()
    }

    pub fn area(&self) -> T {
        self.size.width() * self.size.height()
    }
}

impl<T> Rectangle<T>
where
    T: Copy + Num + PartialOrd,
{
    pub fn contains(&self, point: Point<T>) -> bool {
        self.origin.horizontal() <= point.horizontal()
            && self.origin.vertical() <= point.vertical()
            && self.origin.horizontal() + self.size.width() >= point.horizontal()
            && self.origin.vertical() + self.size.height() <= point.vertical()
    }
}

impl From<Rectangle<NonZeroU32>> for Rectangle<u32> {
    fn from(value: Rectangle<NonZeroU32>) -> Self {
        Rectangle { origin: value.origin.into(), size: value.size.into() }
    }
}

impl From<Rectangle<u32>> for Rectangle<f32> {
    fn from(value: Rectangle<u32>) -> Self {
        Rectangle { origin: value.origin.into(), size: value.size.into() }
    }
}
