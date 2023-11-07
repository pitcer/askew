use std::ops::{Range, RangeInclusive};

use crate::config::CanvasConfig;
use num_traits::{Num, NumCast};

pub mod request;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Samples {
    samples: usize,
}

#[derive(Debug)]
pub struct EquallySpacedIterator {
    start: f32,
    delta: f32,
    length: f32,
    iterator: Range<usize>,
}

impl Samples {
    #[must_use]
    pub fn new(samples: usize) -> Self {
        Self { samples }
    }

    #[must_use]
    pub fn equally_spaced(&self, range: RangeInclusive<f32>) -> EquallySpacedIterator {
        EquallySpacedIterator::new(range, self.samples)
    }

    #[must_use]
    pub fn samples(&self) -> usize {
        self.samples
    }
}

impl EquallySpacedIterator {
    #[must_use]
    pub fn new(range: RangeInclusive<f32>, samples: usize) -> Self {
        let start = *range.start();
        let delta = *range.end() - start;
        let length = num_traits::cast::<usize, f32>(samples - 1)
            .expect("samples should be representable by the given type");
        let iterator = 0..samples;
        Self { start, delta, length, iterator }
    }
}

impl Iterator for EquallySpacedIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.iterator.next()?;
        let index = num_traits::cast::<usize, f32>(index)
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
