pub use declare::*;

use crate::event::input::command;

pub mod declare;
pub mod handler;
mod macros;

#[derive(Debug)]
pub enum FrameEvent {
    ToggleConvexHull(input::ToggleConvexHull),
    ChangeWeight(input::ChangeWeight),
    MovePoint(input::MovePoint),
    AddPoint(canvas::AddPoint),
    AddCurve(canvas::AddCurve),
    Delete(input::Delete),
    ChangeIndex(input::ChangeIndex),

    EnterCommand(command::EnterCommand),
    ReceiveCharacter(command::ReceiveCharacter),
    ExecuteCommand(command::ExecuteCommand),
    ExitMode(command::ExitMode),
    ChangeMode(input::ChangeMode),
}

pub trait Event {
    type Return;
}

pub type HandlerResult<E> = anyhow::Result<<E as Event>::Return, Error>;

pub trait EventHandler<E>
where
    E: Event,
{
    fn handle(&mut self, event: E) -> HandlerResult<E>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unimplemented handler")]
    Unimplemented,
    #[error("no such point: {0}")]
    NoSuchPoint(PointId),
    #[error("other error: {0}")]
    Other(anyhow::Error),
}

pub type PointId = usize;

#[derive(Debug)]
pub enum Change {
    Decrease,
    Increase,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
