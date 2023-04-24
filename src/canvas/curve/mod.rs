use crate::canvas::curve::polyline::Polyline;
use nalgebra::Point2;

pub mod polyline;

pub enum Curve {
    Polyline(Polyline),
}

impl Curve {
    pub fn line_approx_points(&self) -> impl Iterator<Item = &Point2<f32>> {
        match self {
            Curve::Polyline(polyline) => polyline.line_approx_points(),
        }
    }

    pub fn add_point(&mut self, point: Point2<f32>) {
        match self {
            Curve::Polyline(polyline) => polyline.add_point(point),
        }
    }
}
