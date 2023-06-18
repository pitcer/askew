use anyhow::Result;

use crate::canvas::Canvas;
use crate::event::canvas::{GetConvexHull, SetConvexHull};
use crate::event::curve::control_points::SetInterpolationNodes;
use crate::event::curve::SetSamples;
use crate::event::{EventHandler, InputEvent};
use crate::ui::command::parser::{Command, Get, Set, Toggle};

#[derive(Debug)]
pub struct CommandInterpreter<'a> {
    properties: &'a mut Canvas,
}

impl<'a> CommandInterpreter<'a> {
    pub fn new(properties: &'a mut Canvas) -> Self {
        Self { properties }
    }

    pub fn interpret(&mut self, command: Command) -> Result<Option<InputEvent>, Error> {
        match command {
            Command::Get(get) => self.interpret_get(get),
            Command::Set(set) => self.interpret_set(set).map_err(Error::OtherError)?,
            Command::Toggle(toggle) => self.interpret_toggle(toggle).map_err(Error::OtherError)?,
        }
        Err(Error::UnknownCommand)
    }

    fn interpret_get(&mut self, get: Get) {
        match get {
            Get::ConvexHull => {}
            Get::InterpolationNodes => {}
            Get::Samples => {}
        }
    }

    fn interpret_set(&mut self, set: Set) -> Result<()> {
        match set {
            Set::ConvexHull(value) => self
                .properties
                .event_handler()
                .handle(SetConvexHull(value))?,
            Set::InterpolationNodes(value) => self
                .properties
                .event_handler()
                .handle(SetInterpolationNodes::new(value))?,
            Set::Samples(value) => self.properties.event_handler().handle(SetSamples(value))?,
        }
        Ok(())
    }

    fn interpret_toggle(&mut self, toggle: Toggle) -> Result<()> {
        match toggle {
            Toggle::ConvexHull => {
                let value = self.properties.event_handler().handle(GetConvexHull)?;
                self.properties
                    .event_handler()
                    .handle(SetConvexHull(!value))?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown command")]
    UnknownCommand,
    #[error("other error: {0}")]
    OtherError(anyhow::Error),
}
