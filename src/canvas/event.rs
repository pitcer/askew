use crate::canvas::geometry::vector::Vector;

pub enum CanvasEvent {
    ChangeCurrentIndex(i32),
    ChangeWeight(f32),
    ToggleConvexHull,
    DeleteCurrentPoint,
    MoveCurrentPoint(Vector<f32>),
}
