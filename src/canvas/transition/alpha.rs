use crate::canvas::transition::private::TransitionDetails;
use crate::canvas::transition::progress::mapping::LinearMapping;
use crate::canvas::transition::progress::Progress;
use crate::config::rgb::Alpha;

#[derive(Debug, Copy, Clone)]
pub struct AlphaTransitionDetails;

impl TransitionDetails for AlphaTransitionDetails {
    type Property = Alpha;
    type Mapping = LinearMapping;

    fn create_mapping(from: Self::Property, to: Self::Property) -> Self::Mapping {
        let from = f32::from(from);
        let to = f32::from(to);
        LinearMapping::new(from, to)
    }

    fn from_progress(progress: Progress, mapping: &Self::Mapping) -> Self::Property {
        let progress = mapping.map(progress);
        let alpha = progress.round() as u8;
        Alpha::new(alpha)
    }
}
