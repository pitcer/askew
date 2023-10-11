use crate::canvas::v2::visual_path::point::VisualPoint;
use crate::canvas::v2::visual_path::line::VisualLine;

#[derive(Debug, Clone)]
pub struct BasePolyline {
    line: VisualLine,
    points: VisualPoint,
}

impl BasePolyline {
    pub fn new(line: VisualLine, points: VisualPoint) -> Self {
        Self { line, points }
    }
}
