use std::ops::{Range, RangeInclusive};

use num_traits::{Num, NumCast};

use crate::config::CanvasConfig;

pub mod request;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Samples {
    samples: usize,
}

#[derive(Debug)]
pub struct EquallySpacedIterator<T> {
    start: T,
    delta: T,
    length: T,
    iterator: Range<usize>,
}

impl Samples {
    #[must_use]
    pub fn new(samples: usize) -> Self {
        Self { samples }
    }

    #[must_use]
    pub fn equally_spaced<T>(&self, range: RangeInclusive<T>) -> EquallySpacedIterator<T>
    where
        T: Copy + Num + NumCast,
    {
        EquallySpacedIterator::new(range, self.samples)
    }

    #[must_use]
    pub fn samples(&self) -> usize {
        self.samples
    }
}

impl<T> EquallySpacedIterator<T>
where
    T: Copy + Num + NumCast,
{
    #[must_use]
    pub fn new(range: RangeInclusive<T>, samples: usize) -> Self {
        debug_assert!(samples >= 2);

        let start = *range.start();
        let delta = *range.end() - start;
        let length = num_traits::cast::<usize, T>(samples - 1)
            .expect("samples should be representable by the given type");
        let iterator = 0..samples;
        Self { start, delta, length, iterator }
    }
}

impl<T> Iterator for EquallySpacedIterator<T>
where
    T: Copy + Num + NumCast,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.iterator.next()?;
        let index = num_traits::cast::<usize, T>(index)
            .expect("index should be representable by the given type");
        Some(self.start + (index * self.delta) / self.length)
    }
}

impl Default for Samples {
    fn default() -> Self {
        Self::new(2)
    }
}

impl From<&CanvasConfig> for Samples {
    fn from(value: &CanvasConfig) -> Self {
        Self { samples: value.curve_samples as usize }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_samples_iterator() {
        let mut iterator = EquallySpacedIterator::new(0.0..=1.0, 2);
        assert_eq!(Some(0.0), iterator.next());
        assert_eq!(Some(1.0), iterator.next());
        assert!(iterator.next().is_none());
    }

    #[test]
    fn three_samples_iterator() {
        let mut iterator = EquallySpacedIterator::new(0.0..=1.0, 3);
        assert_eq!(Some(0.0), iterator.next());
        assert_eq!(Some(0.5), iterator.next());
        assert_eq!(Some(1.0), iterator.next());
        assert!(iterator.next().is_none());
    }
}
