use std::num::NonZeroU32;

#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
    horizontal: T,
    vertical: T,
}

impl<T> Point<T> {
    pub fn new(horizontal: T, vertical: T) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

impl<T> Point<T>
where
    T: Copy,
{
    pub fn horizontal(&self) -> T {
        self.horizontal
    }

    pub fn vertical(&self) -> T {
        self.vertical
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(value: Point<T>) -> Self {
        (value.horizontal, value.vertical)
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Point {
            horizontal: value.0,
            vertical: value.1,
        }
    }
}

impl From<Point<NonZeroU32>> for Point<u32> {
    fn from(value: Point<NonZeroU32>) -> Self {
        Point {
            horizontal: value.horizontal.get(),
            vertical: value.vertical.get(),
        }
    }
}

impl From<Point<u32>> for Point<f32> {
    fn from(value: Point<u32>) -> Self {
        Point {
            horizontal: value.horizontal as f32,
            vertical: value.vertical as f32,
        }
    }
}
