pub use declare::*;

pub mod declare;
pub mod macros;

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

pub trait UnimplementedHandler<E>: EventHandler<E>
where
    E: Event,
{
}

pub trait DelegateEventHandler<E>
where
    E: Event,
{
    type Delegate<'a>: EventHandler<E>
    where
        Self: 'a;

    fn delegate_handler(&mut self) -> Self::Delegate<'_>;

    fn delegate(&mut self, event: E) -> HandlerResult<E> {
        self.delegate_handler().handle(event)
    }
}

pub trait DelegateEvent<E>
where
    E: Event,
{
    fn delegate(&mut self, event: E) -> HandlerResult<E>;
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
