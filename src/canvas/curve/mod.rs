use crate::canvas::curve::bezier::Bezier;
use crate::canvas::curve::convex_hull::ConvexHull;
use crate::canvas::curve::interpolation::Interpolation;
use crate::canvas::curve::polyline::Polyline;
use crate::canvas::curve::trochoid::Trochoid;
use crate::canvas::geometry::point::Point;

pub mod bezier;
pub mod convex_hull;
pub mod interpolation;
pub mod polyline;
pub mod trochoid;

#[macro_export]
macro_rules! curve_apply {
    ($curve_ident:expr => |$curve:ident| $function:expr) => {{
        match $curve_ident {
            Curve::Polyline($curve) => $function,
            Curve::Interpolation($curve) => $function,
            Curve::Bezier($curve) => $function,
            Curve::ConvexHull($curve) => $function,
            Curve::Trochoid($curve) => $function,
        }
    }};
}

pub type CurvePoint = Point<f32>;

#[derive(Debug)]
pub enum Curve {
    Polyline(Polyline),
    Interpolation(Interpolation),
    Bezier(Bezier),
    ConvexHull(ConvexHull),
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
