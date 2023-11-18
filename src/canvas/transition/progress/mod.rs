use crate::canvas::samples::EquallySpacedIterator;
use crate::canvas::transition::progress::cubic_bezier::CubicBezier;

pub mod cubic_bezier;
pub mod mapping;

#[derive(Debug, Copy, Clone)]
pub struct Progress(f32);

#[derive(Debug)]
pub struct ProgressIterator {
    function: CubicBezier,
    steps: EquallySpacedIterator<f32>,
}

impl ProgressIterator {
    #[must_use]
    pub fn new(function: CubicBezier, steps: u32) -> Self {
        debug_assert!(steps >= 1);

        let mut steps = EquallySpacedIterator::new(0.0..=1.0, steps as usize + 1);
        let _ = steps.next();
        Self { function, steps }
    }
}

impl Iterator for ProgressIterator {
    type Item = Progress;

    fn next(&mut self) -> Option<Self::Item> {
        let argument = self.steps.next()?;
        let progress = self.function.evaluate(argument);
        Some(Progress(progress))
    }
}
