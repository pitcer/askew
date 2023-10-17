use crate::canvas::curve::CurveKind;

pub enum ShapeKind {
    Curve(CurveKind),
    Square,
    Rectangle,
    Circle,
    Oval,
}

pub struct Shape {}
