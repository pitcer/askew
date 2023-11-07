use crate::canvas::transition::bezier::{BezierTransition, CubicBezier};
use crate::canvas::transition::Transition;
use crate::config::rgb::Alpha;

#[derive(Debug)]
pub struct AlphaTransition {
    transition: BezierTransition,
    current: Alpha,
}

impl AlphaTransition {
    #[must_use]
    pub fn new(from: Alpha, to: Alpha, function: CubicBezier, steps: u32) -> Self {
        let (from, current) = (f32::from(from), from);
        let to = f32::from(to);
        let transition = BezierTransition::new(from, to, function, steps);
        Self { transition, current }
    }
}

impl Transition for AlphaTransition {
    type Item = Alpha;

    fn next_step(&mut self) -> Option<&Self::Item> {
        let progress = self.transition.next()?;
        let alpha = progress.round() as u8;
        self.current = Alpha::new(alpha);
        Some(&self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_step() {
        let mut transition =
            AlphaTransition::new(Alpha::TRANSPARENT, Alpha::OPAQUE, CubicBezier::LINEAR, 1);
        assert_eq!(Some(Alpha::OPAQUE), transition.next_step().copied());
        assert!(transition.next_step().is_none());
    }

    #[test]
    fn two_steps() {
        let mut transition =
            AlphaTransition::new(Alpha::TRANSPARENT, Alpha::OPAQUE, CubicBezier::LINEAR, 2);
        assert_eq!(Some(Alpha::new(128)), transition.next_step().copied());
        assert_eq!(Some(Alpha::OPAQUE), transition.next_step().copied());
        assert!(transition.next_step().is_none());
    }

    #[test]
    fn three_steps() {
        let mut transition =
            AlphaTransition::new(Alpha::TRANSPARENT, Alpha::OPAQUE, CubicBezier::LINEAR, 3);
        assert_eq!(Some(Alpha::new(85)), transition.next_step().copied());
        assert_eq!(Some(Alpha::new(170)), transition.next_step().copied());
        assert_eq!(Some(Alpha::OPAQUE), transition.next_step().copied());
        assert!(transition.next_step().is_none());
    }

    #[test]
    fn one_step_decreasing() {
        let mut transition =
            AlphaTransition::new(Alpha::OPAQUE, Alpha::TRANSPARENT, CubicBezier::LINEAR, 1);
        assert_eq!(Some(Alpha::TRANSPARENT), transition.next_step().copied());
        assert!(transition.next_step().is_none());
    }

    #[test]
    fn two_steps_decreasing() {
        let mut transition =
            AlphaTransition::new(Alpha::OPAQUE, Alpha::TRANSPARENT, CubicBezier::LINEAR, 2);
        assert_eq!(Some(Alpha::new(128)), transition.next_step().copied());
        assert_eq!(Some(Alpha::TRANSPARENT), transition.next_step().copied());
        assert!(transition.next_step().is_none());
    }

    #[test]
    fn three_steps_decreasing() {
        let mut transition =
            AlphaTransition::new(Alpha::OPAQUE, Alpha::TRANSPARENT, CubicBezier::LINEAR, 3);
        assert_eq!(Some(Alpha::new(170)), transition.next_step().copied());
        assert_eq!(Some(Alpha::new(85)), transition.next_step().copied());
        assert_eq!(Some(Alpha::TRANSPARENT), transition.next_step().copied());
        assert!(transition.next_step().is_none());
    }
}
