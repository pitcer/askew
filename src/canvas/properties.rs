#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CanvasProperties {
    pub current_point_index: usize,
    pub current_curve: usize,
}
