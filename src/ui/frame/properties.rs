use std::path::PathBuf;

use crate::config::{Config, SaveFormat};

#[derive(Debug)]
pub struct FrameProperties {
    pub save_format: Option<SaveFormat>,
    pub default_save_path: PathBuf,
}

impl FrameProperties {
    #[must_use]
    pub fn new(config: &Config) -> Self {
        Self {
            save_format: config.save_format,
            default_save_path: PathBuf::from("askew_canvas.json"),
        }
    }
}
