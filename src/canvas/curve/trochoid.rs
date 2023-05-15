use std::borrow::Cow;

use crate::canvas::curve::CurvePoint;
use crate::canvas::geometry::point::Point;

#[derive(Debug)]
pub struct Trochoid {
    samples: u32,
    range: (f32, f32),
    r_1: f32,
    r_2: f32,
    w_1: f32,
    w_2: f32,
}

impl Trochoid {
    pub fn new(samples: u32, range: (f32, f32), r_1: f32, r_2: f32, w_1: f32, w_2: f32) -> Self {
        Self {
            samples,
            range,
            r_1,
            r_2,
            w_1,
            w_2,
        }
    }

    pub fn line_approx_points(&self) -> Option<impl Iterator<Item = CurvePoint> + '_> {
        let delta = self.range.1 - self.range.0;
        let times = (0..=self.samples)
            .map(move |index| self.range.0 + (index as f32 * delta) / self.samples as f32);
        let x = move |t| self.r_1 * f32::cos(self.w_1 * t) + self.r_2 * f32::cos(self.w_2 * t);
        let y = move |t| self.r_1 * f32::sin(self.w_1 * t) + self.r_2 * f32::sin(self.w_2 * t);
        let map = times.map(move |t| Point::new(x(t) * 200.0 + 250.0, y(t) * 200.0 + 250.0));
        Some(map)
    }

    pub fn add_point(&mut self, _point: CurvePoint) {}

    pub fn points(&self) -> Cow<'_, [CurvePoint]> {
        Cow::Borrowed(&[])
    }
}
