use crate::canvas::control_points::point::PointContainer;
use crate::canvas::math::point::Point;

#[derive(Debug, Copy, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct WeightedPoint<T, W> {
    point: Point<T>,
    weight: W,
}

impl<T, W> WeightedPoint<T, W> {
    pub fn new(point: Point<T>, weight: W) -> Self {
        Self { point, weight }
    }

    pub fn weight(self) -> W {
        self.weight
    }

    pub fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

impl<T, W> PointContainer<T> for WeightedPoint<T, W>
where
    T: Copy,
{
    fn into_point(self) -> Point<T> {
        self.point
    }

    fn point_mut(&mut self) -> &mut Point<T> {
        &mut self.point
    }
}
