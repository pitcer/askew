use std::f32::consts;

use anyhow::Result;

use crate::canvas::math::vector::Vector;
use crate::config::CurveType;
use crate::event::canvas::{GetConvexHull, MoveCurve, RotateCurve, SetConvexHull, SetCurveType};
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::DelegateEventHandler;
use crate::ui::command::message::Message;
use crate::ui::command::parser::{Command, Get, Set, Toggle};
use crate::ui::frame::event_handler::CommandEventHandler;
use crate::ui::frame::Frame;
use crate::ui::state::ProgramState;

#[derive(Debug)]
pub struct CommandInterpreter<'a> {
    state: ProgramState<'a>,
}

type InterpretResult<E = anyhow::Error> = Result<Option<Message>, E>;

impl<'a> CommandInterpreter<'a> {
    #[must_use]
    pub fn new(state: ProgramState<'a>) -> Self {
        Self { state }
    }

    fn command_handler(&mut self) -> CommandEventHandler<'_> {
        self.state.frame.event_handler(self.state.mode)
    }

    pub fn interpret(&mut self, command: Command<'_>) -> Result<Option<Message>, Error> {
        let result = match command {
            Command::Get(get) => self.interpret_get(get),
            Command::Set(set) => self.interpret_set(set),
            Command::Toggle(toggle) => self.interpret_toggle(toggle),
            Command::Rotate(angle) => self.interpret_rotate(angle),
            Command::Move(horizontal, vertical) => self.interpret_move(horizontal, vertical),
            Command::Save(path) => self.interpret_save(path),
            Command::Open(path) => self.interpret_open(path),
            Command::SetCurveType(curve_type) => self.interpret_set_curve_type(curve_type),
        };
        result.map_err(Error::OtherError)
    }

    fn interpret_get(&mut self, get: Get) -> InterpretResult {
        let mut handler = self.command_handler();

        let message = match get {
            Get::ConvexHull => {
                let convex_hull = handler.delegate(GetConvexHull)?;
                format!("{convex_hull}")
            }
            Get::InterpolationNodes => {
                let nodes = handler.delegate(GetInterpolationNodes)?;
                format!("{nodes:?}")
            }
            Get::Samples => {
                let samples = handler.delegate(GetSamples)?;
                format!("{samples}")
            }
        };
        Ok(Some(Message::info(message)))
    }

    fn interpret_set(&mut self, set: Set) -> InterpretResult {
        let mut handler = self.command_handler();

        match set {
            Set::ConvexHull(value) => handler.delegate(SetConvexHull(value))?,
            Set::InterpolationNodes(value) => {
                handler.delegate(SetInterpolationNodes::new(value))?;
            }
            Set::Samples(value) => handler.delegate(SetSamples(value))?,
        }
        Ok(None)
    }

    fn interpret_toggle(&mut self, toggle: Toggle) -> InterpretResult {
        let mut handler = self.command_handler();

        match toggle {
            Toggle::ConvexHull => {
                let value = handler.delegate(GetConvexHull)?;
                handler.delegate(SetConvexHull(!value))?;
            }
            Toggle::ControlLine => {
                self.state.frame.canvas_mut().properties_mut().control_line =
                    !self.state.frame.canvas().properties().control_line;
            }
        }
        Ok(None)
    }

    fn interpret_rotate(&mut self, angle: u16) -> InterpretResult {
        let mut handler = self.command_handler();
        let radians = consts::PI * f32::from(angle) / 180.0;
        handler.delegate(RotateCurve::new(radians))?;
        Ok(Some(Message::info(format!("Curve rotated by {angle} deg"))))
    }

    fn interpret_move(&mut self, horizontal: f32, vertical: f32) -> InterpretResult {
        let mut handler = self.command_handler();
        let shift = Vector::new(horizontal, vertical);
        handler.delegate(MoveCurve::new(shift))?;
        Ok(Some(Message::info(format!(
            "Curve moved by ({horizontal}, {vertical})"
        ))))
    }

    fn interpret_save(&mut self, path: Option<&str>) -> InterpretResult {
        let path = path.unwrap_or_else(|| &self.state.frame.properties().default_save_path);
        self.state.frame.save_canvas(path)?;
        Ok(Some(Message::info(format!("Project saved into {path}"))))
    }

    fn interpret_open(&mut self, path: Option<&str>) -> InterpretResult {
        let path = path.unwrap_or_else(|| &self.state.frame.properties().default_save_path);
        let canvas = Frame::open_canvas(path)?;
        let message = Message::info(format!("Project opened from {path}"));
        self.state.frame.load_canvas(canvas);
        Ok(Some(message))
    }

    fn interpret_set_curve_type(&mut self, curve_type: CurveType) -> InterpretResult {
        self.command_handler().delegate(SetCurveType(curve_type))?;
        Ok(Some(Message::info(format!(
            "Set curve type to {curve_type}"
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
