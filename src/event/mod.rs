use winit::dpi::PhysicalPosition;

use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::math::vector::Vector;

pub mod handler;

pub enum CanvasEvent {
    ChangeCurrentIndex(i32),
    ChangeWeight(f32),
    ToggleConvexHull,
    DeleteCurrentPoint,
    MoveCurrentPoint(Vector<f32>),
    AddPoint(PhysicalPosition<f64>),
    Resize { area: Rectangle<f32> },
}
