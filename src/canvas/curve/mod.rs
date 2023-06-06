use std::fmt::{Debug, Display, Formatter};

use crate::canvas::curve::control_points::ControlPointsCurve;
use crate::canvas::curve::formula::FormulaCurve;

pub mod control_points;
pub mod curve_path;
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
