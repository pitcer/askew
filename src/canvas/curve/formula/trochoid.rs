use tiny_skia::Path;

use crate::canvas::curve::curve_path;
use crate::canvas::curve::curve_path::{CurvePath, ToPath};
use crate::canvas::math::point::Point;

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
}

impl ToPath for Trochoid {
    fn to_path(&self) -> Option<Path> {
        let x = move |t| self.r_1 * f32::cos(self.w_1 * t) + self.r_2 * f32::cos(self.w_2 * t);
        let y = move |t| self.r_1 * f32::sin(self.w_1 * t) + self.r_2 * f32::sin(self.w_2 * t);
        let path = curve_path::equally_spaced(self.range.0..=self.range.1, self.samples as usize)
            .map(move |t| Point::new(x(t) * 200.0 + 250.0, y(t) * 200.0 + 250.0));
        let path = CurvePath::new(path);
        path.into_skia_path()
    }
}
