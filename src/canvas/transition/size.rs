use std::marker::PhantomData;

use crate::canvas::transition::private::TransitionDetails;
use crate::canvas::transition::progress::mapping::LinearMapping;
use crate::canvas::transition::progress::Progress;

#[derive(Debug, Copy, Clone)]
pub struct SizeTransitionDetails<T>(PhantomData<T>);

impl<T> TransitionDetails for SizeTransitionDetails<T>
where
    T: Copy,
    f32: From<T>,
    T: From<f32>,
{
    type Property = T;
    type Mapping = LinearMapping;

    fn create_mapping(from: Self::Property, to: Self::Property) -> Self::Mapping {
        let from = f32::from(from);
        let to = f32::from(to);
        LinearMapping::new(from, to)
    }

    fn from_progress(progress: Progress, mapping: &Self::Mapping) -> Self::Property {
        let progress = mapping.map(progress);
        T::from(progress)
    }
}
