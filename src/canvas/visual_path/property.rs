use crate::canvas::math::point::Point;
use crate::canvas::transition::progress::cubic_bezier::CubicBezier;
use crate::canvas::transition::{
    AlphaTransition, ColorTransition, PointTransition, SizeTransition, Transition,
};
use crate::config::rgb::{Alpha, Rgb};

pub type AlphaProperty = Property<Alpha, AlphaTransition>;
pub type ColorProperty = Property<Rgb, ColorTransition>;
pub type PointProperty<T> = Property<Point<T>, PointTransition<T>>;
pub type SizeProperty<T> = Property<T, SizeTransition<T>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Property<P, T> {
    property: P,

    // For some reason the value for default is required here. This is probably a serde bug.
    #[serde(skip, default = "Default::default")]
    transition: Option<T>,
}

impl<P, T> Property<P, T>
where
    P: Copy,
    T: Transition<Property = P>,
{
    #[must_use]
    pub fn new(property: P) -> Self {
        Self { property, transition: None }
    }

    pub fn begin_transition(&mut self, goal: P, function: CubicBezier, steps: u32) {
        let transition = T::new(self.property, goal, function, steps);
        self.transition = Some(transition);
    }

    pub fn step(&mut self) {
        let Some(transition) = &mut self.transition else { return };
        if let Some(property) = transition.step() {
            self.property = property;
        } else {
            self.transition = None;
        }
    }

    pub fn value(&self) -> P {
        self.property
    }
}
