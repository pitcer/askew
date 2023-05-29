use crate::canvas::curve::control_points::ControlPointsCurve;
use crate::canvas::curve::curve_path::ToPath;
use crate::canvas::curve::formula::FormulaCurve;
use tiny_skia_path::Path;

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

// impl ToPath for Curve {
//     fn to_path(&self) -> Option<Path> {
//         enum_apply!(self, Curve::ControlPoints | Curve::Formula => |curve| curve.to_path())
//     }
// }
