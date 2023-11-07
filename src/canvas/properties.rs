#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CanvasState {
    pub current_point_index: usize,
    pub current_curve: usize,
}
