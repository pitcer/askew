use std::borrow::Cow;
use crate::canvas::curve::CurvePoint;
use crate::canvas::math;

#[derive(Debug)]
pub struct Bezier {
    points: Vec<CurvePoint>,
    samples: u32,
}

impl Bezier {
    pub fn new(points: Vec<CurvePoint>, samples: u32) -> Self {
        Self { points, samples }
    }

    pub fn line_approx_points(&self) -> Option<impl Iterator<Item = CurvePoint> + '_> {
        if self.points.len() < 2 {
            return None;
        }

        let times = (0..self.samples).map(|index| index as f32 / (self.samples - 1) as f32);
        Some(times.map(|t| self.bezier(t)))
    }

    pub fn add_point(&mut self, point: CurvePoint) {
        self.points.push(point)
    }

    pub fn points(&self) -> Cow<'_, [CurvePoint]> {
        Cow::from(&self.points)
    }

    fn bezier(&self, t: f32) -> CurvePoint {
        let n = self.points.len() as u32 - 1;
        self.points
            .iter()
            .enumerate()
            .map(|(k, point)| {
                let bernstein = math::bernstein(n, k as u32, t);
                CurvePoint::new(point.horizontal() * bernstein, point.vertical() * bernstein)
            })
            .reduce(|accumulator, point| {
                CurvePoint::new(
                    accumulator.horizontal() + point.horizontal(),
                    accumulator.vertical() + point.vertical(),
                )
            })
            .expect("points should not be empty")
    }
}
