use std::ops::RangeInclusive;

use num_traits::{Num, NumCast};

use crate::canvas::curve::samples::event_handler::SamplesEventHandler;

pub mod event_handler;

#[derive(Debug)]
pub struct Samples {
    samples: usize,
}

impl Samples {
    pub fn new(samples: usize) -> Self {
        Self { samples }
    }

    pub fn event_handler(&mut self) -> SamplesEventHandler<'_> {
        SamplesEventHandler::new(self)
    }

    pub fn equally_spaced<T>(&self, range: RangeInclusive<T>) -> impl ExactSizeIterator<Item = T>
    where
        T: Copy + Num + NumCast,
    {
        let range_start = *range.start();
        let delta = *range.end() - range_start;
        let length = num_traits::cast::<usize, T>(self.samples - 1)
            .expect("samples should be representable by the given type");
        (0..self.samples).map(move |index| {
            let index = num_traits::cast::<usize, T>(index)
                .expect("index should be representable by the given type");
            range_start + (index * delta) / length
        })
    }
}
