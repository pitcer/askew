use crate::config::rgb::Rgb;
use crate::config::Config;

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
    pub fn from_config(config: &Config) -> ColorScheme {
        Self {
            background_color: config.ui_background_color,
            status_bar_color: config.ui_status_bar_color,
            command_bar_color: config.ui_command_bar_color,
            text_color: config.ui_text_color,
            text_error_color: config.ui_text_error_color,
        }
    }
}
