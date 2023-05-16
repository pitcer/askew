use std::borrow::Cow;
use crate::canvas::curve::CurvePoint;

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
}
