use crate::canvas::curve::interpolation::Interpolation;
use crate::canvas::curve::polyline::Polyline;
use crate::canvas::curve::trochoid::Trochoid;
use crate::canvas::geometry::point::Point;

pub mod interpolation;
pub mod polyline;
pub mod trochoid;

#[macro_export]
macro_rules! curve_apply {
    ($curve_ident:expr => |$curve:ident| $function:expr) => {{
        match $curve_ident {
            Curve::Polyline($curve) => $function,
            Curve::Interpolation($curve) => $function,
            Curve::Trochoid($curve) => $function,
        }
    }};
}

pub type CurvePoint = Point<f32>;

pub enum Curve {
    Polyline(Polyline),
    Interpolation(Interpolation),
    Trochoid(Trochoid),
}

impl Curve {
    pub fn add_point(&mut self, point: CurvePoint) {
        curve_apply!(self => |curve| curve.add_point(point))
    }

    pub fn points(&self) -> &[CurvePoint] {
        curve_apply!(self => |curve| curve.points())
    }
}
