use crate::config::{Config, SaveFormat};

#[derive(Debug)]
pub struct FrameProperties {
    pub save_format: Option<SaveFormat>,
    pub default_save_path: String,
}

impl FrameProperties {
    #[must_use]
    pub fn new(config: &Config) -> Self {
        Self {
            save_format: config.save_format,
            default_save_path: "askew_canvas.json".to_owned(),
        }
    }
}
