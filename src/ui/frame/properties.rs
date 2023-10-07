use std::path::PathBuf;

use crate::config::FrameConfig;

#[derive(Debug)]
pub struct FrameProperties {
    // TODO: add default image save path (and read that from config)
    pub default_save_path: PathBuf,
}

impl FrameProperties {
    #[must_use]
    pub fn new(_config: FrameConfig) -> Self {
        Self { default_save_path: PathBuf::from("askew_canvas.json") }
    }
}
