use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;

use num_traits::{Num, NumCast};

use crate::canvas::curve::control_points::ControlPointsCurveKind;
use crate::canvas::curve::formula::FormulaCurveKind;

pub mod control_points;
pub mod converter;
pub mod formula;

#[derive(Debug)]
pub enum CurveKind {
    ControlPoints(ControlPointsCurveKind),
    Formula(FormulaCurveKind),
}

impl CurveKind {
    pub fn samples_mut(&mut self) -> Option<&mut u32> {
        match self {
            CurveKind::ControlPoints(curve) => curve.samples_mut(),
            CurveKind::Formula(curve) => curve.samples_mut(),
        }
    }
}

impl Display for CurveKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CurveKind::ControlPoints(curve) => Display::fmt(curve, f),
            CurveKind::Formula(curve) => Display::fmt(curve, f),
        }
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
