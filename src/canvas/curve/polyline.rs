use std::borrow::Cow;
use crate::canvas::curve::CurvePoint;
use crate::canvas::geometry::vector::Vector;

#[derive(Debug)]
pub struct Polyline {
    points: Vec<CurvePoint>,
}

impl Polyline {
    pub fn new(points: Vec<CurvePoint>) -> Self {
        Self { points }
    }

    pub fn line_approx_points(&self) -> Option<impl Iterator<Item = CurvePoint> + '_> {
        Some(self.points.iter().copied())
    }

    pub fn add_point(&mut self, point: CurvePoint) {
        self.points.push(point)
    }

    pub fn remove_point(&mut self, index: usize) {
        self.points.remove(index);
    }

    pub fn points(&self) -> Cow<'_, [CurvePoint]> {
        Cow::from(&self.points)
    }

    pub fn move_point(&mut self, index: usize, vector: Vector<f32>) {
        if let Some(point) = self.points.get_mut(index) {
            *point = *point + vector
        }
    }
}
