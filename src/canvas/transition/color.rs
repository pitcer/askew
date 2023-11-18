use crate::canvas::transition::private::TransitionDetails;
use crate::canvas::transition::progress::mapping::LinearMapping;
use crate::canvas::transition::progress::Progress;
use crate::config::rgb::Rgb;

pub struct ColorTransitionDetails;

impl TransitionDetails for ColorTransitionDetails {
    type Property = Rgb;
    type Mapping = (LinearMapping, LinearMapping, LinearMapping);

    fn create_mapping(from: Self::Property, to: Self::Property) -> Self::Mapping {
        let red_from = f32::from(from.red());
        let green_from = f32::from(from.green());
        let blue_from = f32::from(from.blue());
        let red_to = f32::from(to.red());
        let green_to = f32::from(to.green());
        let blue_to = f32::from(to.blue());
        (
            LinearMapping::new(red_from, red_to),
            LinearMapping::new(green_from, green_to),
            LinearMapping::new(blue_from, blue_to),
        )
    }

    fn from_progress(
        progress: Progress,
        (red_mapping, green_mapping, blue_mapping): &Self::Mapping,
    ) -> Self::Property {
        let red_progress = red_mapping.map(progress);
        let green_progress = green_mapping.map(progress);
        let blue_progress = blue_mapping.map(progress);
        let red = red_progress.round() as u8;
        let green = green_progress.round() as u8;
        let blue = blue_progress.round() as u8;
        Rgb::new(red, green, blue)
    }
}
