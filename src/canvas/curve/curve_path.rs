use std::ops::RangeInclusive;

use num_traits::{Num, NumCast};
use tiny_skia::{Path, PathBuilder};

use crate::canvas::math::point::Point;

pub trait ToPath {
    fn to_path(&self) -> Option<Path>;
}

pub struct CurvePath<I> {
    points: I,
    closed: bool,
}

impl<I> CurvePath<I>
where
    I: ExactSizeIterator<Item = Point<f32>>,
{
    pub fn new(points: I) -> Self {
        Self {
            points,
            closed: false,
        }
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn into_skia_path(mut self) -> Option<Path> {
        let length = self.points.len();
        let mut path = PathBuilder::with_capacity(length, length);

        let point = self.points.next()?;
        path.move_to(point.horizontal(), point.vertical());

        for point in self.points {
            path.line_to(point.horizontal(), point.vertical());
        }

        if self.closed {
            path.close();
        }

        path.finish()
    }
}

pub fn equally_spaced<T>(
    range: RangeInclusive<T>,
    samples: usize,
) -> impl ExactSizeIterator<Item = T>
where
    T: Copy + Num + NumCast,
{
    let range_start = *range.start();
    let delta = *range.end() - range_start;
    let length = num_traits::cast::<usize, T>(samples - 1)
        .expect("samples should be representable by the given type");
    (0..samples).map(move |index| {
        let index = num_traits::cast::<usize, T>(index)
            .expect("index should be representable by the given type");
        range_start + (index * delta) / length
    })
}
