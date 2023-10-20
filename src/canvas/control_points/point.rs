use crate::canvas::control_points::ControlPoints;
use crate::canvas::math::point::Point;

pub type CurveControlPoints = ControlPoints<CurvePoint>;
pub type CurvePoint = Point<f32>;

#[derive(Debug, Copy, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WeightedPoint<T, W> {
    point: Point<T>,
    weight: W,
}

impl<T, W> WeightedPoint<T, W> {
    pub fn new(point: Point<T>, weight: W) -> Self {
        Self { point, weight }
    }

    pub fn point(self) -> Point<T> {
        self.point
    }

    pub fn point_mut(&mut self) -> &mut Point<T> {
        &mut self.point
    }

    pub fn weight(self) -> W {
        self.weight
    }

    pub fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

impl<T, W> AsRef<Point<T>> for WeightedPoint<T, W> {
    fn as_ref(&self) -> &Point<T> {
        &self.point
    }
}

impl<T> AsRef<Point<T>> for Point<T> {
    fn as_ref(&self) -> &Point<T> {
        self
    }
}

impl<T, W> AsMut<Point<T>> for WeightedPoint<T, W> {
    fn as_mut(&mut self) -> &mut Point<T> {
        &mut self.point
    }
}

impl<T> AsMut<Point<T>> for Point<T> {
    fn as_mut(&mut self) -> &mut Point<T> {
        self
    }
}

impl<T, W> From<WeightedPoint<T, W>> for Point<T> {
    fn from(value: WeightedPoint<T, W>) -> Self {
        value.point
    }
}

impl<W> From<WeightedPoint<f32, W>> for tiny_skia::Point {
    fn from(value: WeightedPoint<f32, W>) -> Self {
        value.point.into()
    }
}
