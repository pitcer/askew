use winit::dpi::PhysicalPosition;

use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::math::vector::Vector;
use crate::canvas::mode::Mode;

pub mod handler;

pub enum Event {
    Frame(FrameEvent),
    Canvas(CanvasEvent),
}

pub enum FrameEvent {
    EnterCommand,
    ReceiveCharacter(char),
    ExecuteCommand,
    ExitMode,
}

pub enum CanvasEvent {
    ChangeMode(Mode),
    ChangeIndex(i32),
    Add,
    Delete,
    Curve(CurveEvent),
}

pub enum CurveEvent {
    ChangeCurrentIndex(i32),
    ChangeWeight(f32),
    ToggleConvexHull,
    DeleteCurrentPoint,
    MoveCurrentPoint(Vector<f32>),
    AddPoint(PhysicalPosition<f64>),
    Resize { area: Rectangle<f32> },
}
