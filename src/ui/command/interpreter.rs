use anyhow::Result;
use std::f32::consts;

use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::vector::Vector;
use crate::event::canvas::{GetConvexHull, MoveCurve, RotateCurve, SetConvexHull};
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::EventHandler;
use crate::ui::command::parser::{Command, Get, Set, Toggle};
use crate::ui::command::Message;

#[derive(Debug)]
pub struct CommandInterpreter<'a> {
    handler: CanvasEventHandler<'a>,
}

type InterpretResult = Result<Option<Message>>;

impl<'a> CommandInterpreter<'a> {
    pub fn new(event_handler: CanvasEventHandler<'a>) -> Self {
        Self {
            handler: event_handler,
        }
    }

    pub fn interpret(&mut self, command: Command) -> Result<Option<Message>, Error> {
        let result = match command {
            Command::Get(get) => self.interpret_get(get),
            Command::Set(set) => self.interpret_set(set),
            Command::Toggle(toggle) => self.interpret_toggle(toggle),
            Command::Rotate(angle) => self.interpret_rotate(angle),
            Command::Move(horizontal, vertical) => self.interpret_move(horizontal, vertical),
        };
        result.map_err(Error::OtherError)
    }

    fn interpret_get(&mut self, get: Get) -> InterpretResult {
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
        Ok(Some(Message::info(message)))
    }

    fn interpret_set(&mut self, set: Set) -> InterpretResult {
        match set {
            Set::ConvexHull(value) => self.handler.handle(SetConvexHull(value))?,
            Set::InterpolationNodes(value) => {
                self.handler.handle(SetInterpolationNodes::new(value))?;
            }
            Set::Samples(value) => self.handler.handle(SetSamples(value))?,
        }
        Ok(None)
    }

    fn interpret_toggle(&mut self, toggle: Toggle) -> InterpretResult {
        match toggle {
            Toggle::ConvexHull => {
                let value = self.handler.handle(GetConvexHull)?;
                self.handler.handle(SetConvexHull(!value))?;
            }
        }
        Ok(None)
    }

    fn interpret_rotate(&mut self, angle: u16) -> InterpretResult {
        let radians = consts::PI * f32::from(angle) / 180.0;
        self.handler.handle(RotateCurve::new(radians))?;
        Ok(Some(Message::info(format!("Curve rotated by {angle} deg"))))
    }

    fn interpret_move(&mut self, horizontal: f32, vertical: f32) -> InterpretResult {
        let shift = Vector::new(horizontal, vertical);
        self.handler.handle(MoveCurve::new(shift))?;
        Ok(Some(Message::info(format!(
            "Curve moved by ({horizontal}, {vertical})"
        ))))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown command")]
    UnknownCommand,
    #[error("other error: {0}")]
    OtherError(anyhow::Error),
}
