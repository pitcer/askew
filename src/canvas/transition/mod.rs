use crate::canvas::transition::alpha::AlphaTransitionDetails;
use crate::canvas::transition::color::ColorTransitionDetails;
use crate::canvas::transition::point::PointTransitionDetails;
use crate::canvas::transition::private::TransitionDetails;
use crate::canvas::transition::progress::cubic_bezier::CubicBezier;
use crate::canvas::transition::progress::ProgressIterator;
use crate::canvas::transition::size::SizeTransitionDetails;

pub mod alpha;
pub mod color;
pub mod point;
pub mod progress;
pub mod size;

pub type AlphaTransition = Transition<AlphaTransitionDetails>;
pub type ColorTransition = Transition<ColorTransitionDetails>;
pub type PointTransition<T> = Transition<PointTransitionDetails<T>>;
pub type SizeTransition<T> = Transition<SizeTransitionDetails<T>>;

#[derive(Debug)]
pub struct Transition<T>
where
    T: TransitionDetails,
{
    progress: ProgressIterator,
    mapping: T::Mapping,
}

impl<T> Transition<T>
where
    T: TransitionDetails,
{
    pub fn new(from: T::Property, to: T::Property, function: CubicBezier, steps: u32) -> Self {
        let mapping = T::create_mapping(from, to);
        let progress = ProgressIterator::new(function, steps);
        Self { progress, mapping }
    }

    pub fn step(&mut self) -> Option<T::Property> {
        let progress = self.progress.next()?;
        let property = T::from_progress(progress, &self.mapping);
        Some(property)
    }
}

mod private {
    use crate::canvas::transition::progress::Progress;

    pub trait TransitionDetails {
        type Property;
        type Mapping;

        fn create_mapping(from: Self::Property, to: Self::Property) -> Self::Mapping;

        fn from_progress(progress: Progress, mapping: &Self::Mapping) -> Self::Property;
    }
}

#[cfg(test)]
mod tests {
    use crate::config::rgb::Alpha;

    use super::*;

    #[test]
    fn one_step() {
        let mut transition =
            AlphaTransition::new(Alpha::TRANSPARENT, Alpha::OPAQUE, CubicBezier::LINEAR, 1);
        assert_eq!(Some(Alpha::OPAQUE), transition.step());
        assert!(transition.step().is_none());
    }

    #[test]
    fn two_steps() {
        let mut transition =
            AlphaTransition::new(Alpha::TRANSPARENT, Alpha::OPAQUE, CubicBezier::LINEAR, 2);
        assert_eq!(Some(Alpha::new(128)), transition.step());
        assert_eq!(Some(Alpha::OPAQUE), transition.step());
        assert!(transition.step().is_none());
    }

    #[test]
    fn three_steps() {
        let mut transition =
            AlphaTransition::new(Alpha::TRANSPARENT, Alpha::OPAQUE, CubicBezier::LINEAR, 3);
        assert_eq!(Some(Alpha::new(85)), transition.step());
        assert_eq!(Some(Alpha::new(170)), transition.step());
        assert_eq!(Some(Alpha::OPAQUE), transition.step());
        assert!(transition.step().is_none());
    }

    #[test]
    fn one_step_decreasing() {
        let mut transition =
            AlphaTransition::new(Alpha::OPAQUE, Alpha::TRANSPARENT, CubicBezier::LINEAR, 1);
        assert_eq!(Some(Alpha::TRANSPARENT), transition.step());
        assert!(transition.step().is_none());
    }

    #[test]
    fn two_steps_decreasing() {
        let mut transition =
            AlphaTransition::new(Alpha::OPAQUE, Alpha::TRANSPARENT, CubicBezier::LINEAR, 2);
        assert_eq!(Some(Alpha::new(128)), transition.step());
        assert_eq!(Some(Alpha::TRANSPARENT), transition.step());
        assert!(transition.step().is_none());
    }

    #[test]
    fn three_steps_decreasing() {
        let mut transition =
            AlphaTransition::new(Alpha::OPAQUE, Alpha::TRANSPARENT, CubicBezier::LINEAR, 3);
        assert_eq!(Some(Alpha::new(170)), transition.step());
        assert_eq!(Some(Alpha::new(85)), transition.step());
        assert_eq!(Some(Alpha::TRANSPARENT), transition.step());
        assert!(transition.step().is_none());
    }
}
