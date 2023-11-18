use std::marker::PhantomData;

use crate::canvas::math::point::Point;
use crate::canvas::transition::private::TransitionDetails;
use crate::canvas::transition::progress::mapping::LinearMapping;
use crate::canvas::transition::progress::Progress;

pub struct PointTransitionDetails<T>(PhantomData<T>);

impl<T> TransitionDetails for PointTransitionDetails<T>
where
    T: Copy,
    f32: From<T>,
    T: From<f32>,
{
    type Property = Point<T>;
    type Mapping = (LinearMapping, LinearMapping);

    fn create_mapping(from: Self::Property, to: Self::Property) -> Self::Mapping {
        let horizontal_from = f32::from(from.horizontal());
        let vertical_from = f32::from(from.vertical());
        let horizontal_to = f32::from(to.horizontal());
        let vertical_to = f32::from(to.vertical());
        (
            LinearMapping::new(horizontal_from, horizontal_to),
            LinearMapping::new(vertical_from, vertical_to),
        )
    }

    fn from_progress(
        progress: Progress,
        (horizontal_mapping, vertical_mapping): &Self::Mapping,
    ) -> Self::Property {
        let horizontal_progress = horizontal_mapping.map(progress);
        let vertical_progress = vertical_mapping.map(progress);
        let horizontal = T::from(horizontal_progress);
        let vertical = T::from(vertical_progress);
        Point::new(horizontal, vertical)
    }
}
