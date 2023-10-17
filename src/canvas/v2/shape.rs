use crate::canvas::curve::Curve;

pub enum ShapeKind {
    Curve(Curve),
    Square,
    Rectangle,
    Circle,
    Oval,
}

pub struct Shape {}
