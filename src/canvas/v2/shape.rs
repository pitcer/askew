use crate::canvas::curve::control_points::kind::polyline::Polyline;
use crate::canvas::curve::CurveKind;

pub enum ShapeKind {
    Curve(CurveKind),
    Square,
    Rectangle,
    Circle,
    Oval,
}

pub struct Shape {}
