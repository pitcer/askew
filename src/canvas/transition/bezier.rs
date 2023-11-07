use crate::canvas::math;
use std::ops::RangeInclusive;

use crate::canvas::math::point::Point;
use crate::canvas::samples::{EquallySpacedIterator, Samples};

#[derive(Debug)]
pub struct BezierTransition {
    from: f32,
    to: f32,
    function: CubicBezier,
    steps: RangeInclusive<u32>,
    it: EquallySpacedIterator,
}

#[derive(Debug)]
pub struct CubicBezier {
    point_1: Point<f32>,
    point_2: Point<f32>,
}

impl BezierTransition {
    #[must_use]
    pub fn new(from: f32, to: f32, function: CubicBezier, steps: u32) -> Self {
        let it = EquallySpacedIterator::new(0.0..=1.0, steps as usize);
        let steps = 1..=steps;
        Self { from, to, function, steps, it }
    }
}

impl Iterator for BezierTransition {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.it.next()?;
        let alpha = math::de_casteljau(
            &[
                Point::new(0.0, 0.0),
                self.function.point_1,
                self.function.point_2,
                Point::new(1.0, 1.0),
            ],
            x,
        );
        let steps = self.steps.next()?;
        let alpha = steps as f32 / *self.steps.end() as f32;
        let progress = self.from * (1.0 - alpha) + self.to * alpha;
        Some(progress)
    }
}

impl CubicBezier {
    pub const LINEAR: Self = Self::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));

    #[must_use]
    pub const fn new(point_1: Point<f32>, point_2: Point<f32>) -> Self {
        Self { point_1, point_2 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pts = &[
            Point::new(0.0, 0.0),
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 1.0),
        ];
        let alpha = math::chudy_wozny(pts, 0.0);
        dbg!(alpha);
        let alpha = math::chudy_wozny(pts, 0.5);
        dbg!(alpha);
        let alpha = math::chudy_wozny(pts, 1.0);
        dbg!(alpha);
    }
}
