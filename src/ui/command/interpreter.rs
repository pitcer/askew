use anyhow::Result;

use crate::canvas::event_handler::CanvasEventHandler;
use crate::event::canvas::{GetConvexHull, SetConvexHull};
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::EventHandler;
use crate::ui::command::parser::{Command, Get, Set, Toggle};
use crate::ui::command::Message;

#[derive(Debug)]
pub struct CommandInterpreter<'a> {
    handler: CanvasEventHandler<'a>,
}

impl<'a> CommandInterpreter<'a> {
    pub fn new(event_handler: CanvasEventHandler<'a>) -> Self {
        Self {
            handler: event_handler,
        }
    }

    pub fn interpret(&mut self, command: Command) -> Result<Option<Message>, Error> {
        match command {
            Command::Get(get) => self.interpret_get(get).map(Some).map_err(Error::OtherError),
            Command::Set(set) => self.interpret_set(set).map_err(Error::OtherError),
            Command::Toggle(toggle) => self.interpret_toggle(toggle).map_err(Error::OtherError),
        }
    }

    fn interpret_get(&mut self, get: Get) -> Result<Message> {
        let message = match get {
            Get::ConvexHull => {
                let convex_hull = self.handler.handle(GetConvexHull)?;
                format!("{convex_hull}")
            }
            Get::InterpolationNodes => {
                let nodes = self.handler.handle(GetInterpolationNodes)?;
                format!("{nodes:?}")
            }
            Get::Samples => {
                let samples = self.handler.handle(GetSamples)?;
                format!("{samples}")
            }
        };
        Ok(Message::info(message))
    }

    fn interpret_set(&mut self, set: Set) -> Result<Option<Message>> {
        match set {
            Set::ConvexHull(value) => self.handler.handle(SetConvexHull(value))?,
            Set::InterpolationNodes(value) => {
                self.handler.handle(SetInterpolationNodes::new(value))?;
            }
            Set::Samples(value) => self.handler.handle(SetSamples(value))?,
        }
        Ok(None)
    }

    fn interpret_toggle(&mut self, toggle: Toggle) -> Result<Option<Message>> {
        match toggle {
            Toggle::ConvexHull => {
                let value = self.handler.handle(GetConvexHull)?;
                self.handler.handle(SetConvexHull(!value))?;
            }
        }
        Ok(None)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown command")]
    UnknownCommand,
    #[error("other error: {0}")]
    OtherError(anyhow::Error),
}
