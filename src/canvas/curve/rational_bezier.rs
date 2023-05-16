use crate::canvas::curve::CurvePoint;
use crate::canvas::math;
use std::borrow::Cow;

#[derive(Debug)]
pub struct RationalBezier {
    points: Vec<RationalBezierPoint>,
    samples: u32,
}

impl RationalBezier {
    pub fn new(points: Vec<RationalBezierPoint>, samples: u32) -> Self {
        Self { points, samples }
    }

    pub fn line_approx_points(&self) -> Option<impl Iterator<Item = CurvePoint> + '_> {
        if self.points.len() < 2 {
            return None;
        }

        let times = (0..self.samples).map(|index| index as f32 / (self.samples - 1) as f32);
        Some(times.map(|t| self.rational_bezier(t)))
    }

    pub fn add_point(&mut self, point: CurvePoint) {
        self.add_weighted_point(point, 1.0)
    }

    pub fn add_weighted_point(&mut self, point: CurvePoint, weight: f32) {
        self.points.push(RationalBezierPoint { point, weight })
    }

    pub fn points_mut(&mut self) -> &mut [RationalBezierPoint] {
        &mut self.points
    }

    pub fn points(&self) -> Cow<'_, [CurvePoint]> {
        self.points.iter().map(|point| point.point).collect()
    }

    fn rational_bezier(&self, t: f32) -> CurvePoint {
        let n = self.points.len() as u32 - 1;
        let result = self
            .points
            .iter()
            .enumerate()
            .map(|(k, point)| {
                let bernstein = math::bernstein(n, k as u32, t);
                CurvePoint::new(
                    point.point.horizontal() * bernstein * point.weight,
                    point.point.vertical() * bernstein * point.weight,
                )
            })
            .reduce(|accumulator, point| {
                CurvePoint::new(
                    accumulator.horizontal() + point.horizontal(),
                    accumulator.vertical() + point.vertical(),
                )
            })
            .expect("points should not be empty");
        let divisor = self
            .points
            .iter()
            .enumerate()
            .map(|(k, point)| point.weight * math::bernstein(n, k as u32, t))
            .sum::<f32>();
        CurvePoint::new(result.horizontal() / divisor, result.vertical() / divisor)
    }
}

#[derive(Debug)]
pub struct RationalBezierPoint {
    point: CurvePoint,
    weight: f32,
}

impl RationalBezierPoint {
    pub fn new(point: CurvePoint, weight: f32) -> Self {
        Self { point, weight }
    }

    pub fn change_weight(&mut self, change: f32) {
        if change < 0.0 {
            self.weight /= -change
        } else {
            self.weight *= change
        }
    }
}
