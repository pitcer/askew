use crate::canvas::control_points::ControlPoints;
use crate::canvas::math::point::Point;

pub type CurveControlPoints = ControlPoints<CurvePoint>;
pub type CurvePoint = Point<f32>;

pub trait PointContainer<T> {
    fn into_point(self) -> Point<T>;

    fn point_mut(&mut self) -> &mut Point<T>;
}

impl<T> PointContainer<T> for Point<T>
where
    T: Copy,
{
    fn into_point(self) -> Point<T> {
        self
    }

    fn point_mut(&mut self) -> &mut Point<T> {
        self
    }
}
