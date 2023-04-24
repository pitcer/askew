use std::num::NonZeroU32;
use std::ops::Add;

use crate::canvas::geometry::point::Point;
use crate::canvas::geometry::size::Size;

/// Rectangle represented by origin (left bottom point) and its size.
#[derive(Debug, Copy, Clone)]
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
    T: Copy + Add + PartialOrd,
    <T as Add>::Output: PartialOrd<T>,
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
        Rectangle {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}

impl From<Rectangle<u32>> for Rectangle<f32> {
    fn from(value: Rectangle<u32>) -> Self {
        Rectangle {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}
