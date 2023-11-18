use tinyvec::tiny_vec;

use crate::canvas::math::point::Point;
use crate::canvas::math::polynomial::Polynomial;

#[derive(Debug)]
pub struct CubicBezier {
    point_1: Point<f32>,
    point_2: Point<f32>,
}

impl CubicBezier {
    pub const LINEAR: Self = Self::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));

    #[must_use]
    pub const fn new(point_1: Point<f32>, point_2: Point<f32>) -> Self {
        Self { point_1, point_2 }
    }

    #[must_use]
    pub fn evaluate(&self, argument: f32) -> f32 {
        if argument == 0.0 {
            return 0.0;
        } else if f32::abs(argument - 1.0) < f32::EPSILON {
            return 1.0;
        }

        let horizontal_polynomial =
            Self::polynomial(self.point_1.horizontal(), self.point_2.horizontal(), -argument);
        let root = horizontal_polynomial.find_root(16, argument);
        debug_assert!(
            f32::abs(
                Self::polynomial(self.point_1.horizontal(), self.point_2.horizontal(), 0.0)
                    .evaluate(root)
                    - argument
            ) < 1.0e-3f32
        );

        let vertical_polynomial =
            Self::polynomial(self.point_1.vertical(), self.point_2.vertical(), 0.0);
        vertical_polynomial.evaluate(root)
    }

    fn polynomial(point_1: f32, point_2: f32, shift: f32) -> Polynomial {
        Polynomial::new(tiny_vec![
            shift,
            3.0 * point_1,
            -6.0 * point_1 + 3.0 * point_2,
            1.0 + 3.0 * point_1 - 3.0 * point_2
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate() {
        let bezier = CubicBezier::LINEAR;
        assert!(f32::abs(0.87 - bezier.evaluate(0.87)) < 1.0e-3f32);
        assert!(f32::abs(0.5 - bezier.evaluate(0.5)) < 1.0e-3f32);
        assert!(f32::abs(0.999 - bezier.evaluate(0.999)) < 1.0e-3f32);
        assert!(f32::abs(1.0 - bezier.evaluate(1.0)) < 1.0e-3f32);
        assert!(f32::abs(0.0 - bezier.evaluate(0.0)) < 1.0e-3f32);
        assert!(f32::abs(0.001 - bezier.evaluate(0.001)) < 1.0e-3f32);
    }
}
