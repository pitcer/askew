pub use declare::*;

pub mod declare;
pub mod macros;

pub trait Ret {
    type Return;
}

pub trait Event: Ret {}

pub trait EventMut: Ret {}

pub type HandlerResult<E> = anyhow::Result<<E as Ret>::Return, Error>;

pub trait EventHandler<E>
where
    E: Event,
{
    fn handle(&self, event: E) -> HandlerResult<E>;
}

pub trait EventHandlerMut<E>
where
    E: EventMut,
{
    fn handle_mut(&mut self, event: E) -> HandlerResult<E>;
}

pub trait UnimplementedHandler<E> {}

pub trait DelegateEventHandler<E>
where
    E: Event,
{
    type Delegate<'a>: EventHandler<E>
    where
        Self: 'a;

    fn delegate_handler(&self) -> Self::Delegate<'_>;

    fn delegate(&self, event: E) -> HandlerResult<E> {
        self.delegate_handler().handle(event)
    }
}

pub trait DelegateEventHandlerMut<E>
where
    E: EventMut,
{
    type Delegate<'a>: EventHandlerMut<E>
    where
        Self: 'a;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_>;

    fn delegate_mut(&mut self, event: E) -> HandlerResult<E> {
        self.delegate_handler_mut().handle_mut(event)
    }
}

pub trait DelegateEvent<E>
where
    E: Event,
{
    fn delegate(&self, event: E) -> HandlerResult<E>;
}

pub trait DelegateEventMut<E>
where
    E: EventMut,
{
    fn delegate_mut(&mut self, event: E) -> HandlerResult<E>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unimplemented handler")]
    Unimplemented,
    #[error("no such point: {0}")]
    NoSuchPoint(PointId),
    #[error("no such curve: {0}")]
    NoSuchCurve(usize),
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
