use winit::dpi::PhysicalPosition;

use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::math::vector::Vector;
use crate::canvas::mode::Mode;

pub mod handler;

#[derive(Debug)]
pub enum Event {
    Frame(FrameEvent),
    Canvas(CanvasEvent),
}

#[derive(Debug)]
pub enum FrameEvent {
    EnterCommand,
    ReceiveCharacter(char),
    ExecuteCommand,
    ExitMode,
}

#[derive(Debug)]
pub enum CanvasEvent {
    ChangeMode(Mode),
    ChangeIndex(i32),
    Add,
    Delete,
    Curve(CurveEvent),
}

#[derive(Debug)]
pub enum CurveEvent {
    ChangeCurrentIndex(i32),
    ChangeWeight(f32),
    ToggleConvexHull,
    DeleteCurrentPoint,
    MoveCurrentPoint(Vector<f32>),
    AddPoint(PhysicalPosition<f64>),
    Resize { area: Rectangle<f32> },
}
