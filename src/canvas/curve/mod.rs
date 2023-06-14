use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;

use num_traits::{Num, NumCast};

use crate::canvas::curve::control_points::ControlPointsCurve;
use crate::canvas::curve::formula::FormulaCurve;

pub mod control_points;
pub mod converter;
pub mod formula;

#[macro_export]
macro_rules! enum_apply {
    ($self:expr,
        $($variant:path)|+ => |$variant_name:ident| $function:expr
        $(,$other:pat => $other_expr:expr)*) => {{

        match $self {
            $($variant($variant_name) => $function,)+
            $($other => $other_expr,)*
        }
    }};
}

#[derive(Debug)]
pub enum Curve {
    ControlPoints(ControlPointsCurve),
    Formula(FormulaCurve),
}

impl Display for Curve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Curve::ControlPoints(curve) => Display::fmt(curve, f),
            Curve::Formula(curve) => Display::fmt(curve, f),
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
