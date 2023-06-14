use tiny_skia::{Path, PathBuilder};

use crate::canvas::math::point::Point;

pub trait ToPath {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P>;
}

pub trait PathConverter {
    type Path;

    fn to_path<I, P>(&self, path: CurvePath<I>) -> Option<Self::Path>
    where
        I: ExactSizeIterator<Item = P>,
        P: AsRef<Point<f32>>;
}

pub struct TinySkiaPathConverter;

impl PathConverter for TinySkiaPathConverter {
    type Path = Path;

    fn to_path<I, P>(&self, path: CurvePath<I>) -> Option<Self::Path>
    where
        I: ExactSizeIterator<Item = P>,
        P: AsRef<Point<f32>>,
    {
        let CurvePath { mut points, closed } = path;
        let length = points.len();
        let mut path = PathBuilder::with_capacity(length, length);

        let point = points.next()?;
        let point = point.as_ref();
        path.move_to(point.horizontal(), point.vertical());

        for point in points {
            let point = point.as_ref();
            path.line_to(point.horizontal(), point.vertical());
        }

        if closed {
            path.close();
        }

        path.finish()
    }
}

#[derive(Debug)]
pub struct CurvePath<I> {
    points: I,
    closed: bool,
}

impl<I> CurvePath<I>
where
    I: ExactSizeIterator<Item = Point<f32>>,
{
    pub fn new(points: I, closed: bool) -> Self {
        Self { points, closed }
    }

    pub fn new_open(points: I) -> Self {
        Self::new(points, false)
    }

    pub fn new_closed(points: I) -> Self {
        Self::new(points, true)
    }

    pub fn closed(&self) -> bool {
        self.closed
    }
}
