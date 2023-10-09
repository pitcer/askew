use std::path::PathBuf;

use crate::config::FrameConfig;

#[derive(Debug)]
pub struct FrameProperties {
    pub default_project_save_path: PathBuf,
    pub default_image_save_path: PathBuf,
}

impl FrameProperties {
    #[must_use]
    pub fn new(config: FrameConfig) -> Self {
        Self {
            default_project_save_path: config.default_project_save_path,
            default_image_save_path: config.default_image_save_path,
        }
    }
}
