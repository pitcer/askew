use crate::config::rgb::Rgb;
use crate::config::UiConfig;

#[derive(Debug)]
pub struct ColorScheme {
    pub background_color: Rgb,
    pub status_bar_color: Rgb,
    pub command_bar_color: Rgb,
    pub text_color: Rgb,
    pub text_error_color: Rgb,
}

impl ColorScheme {
    #[must_use]
    pub fn from_config(config: UiConfig) -> ColorScheme {
        Self {
            background_color: config.background_color,
            status_bar_color: config.status_bar_color,
            command_bar_color: config.command_bar_color,
            text_color: config.text_color,
            text_error_color: config.text_error_color,
        }
    }
}
