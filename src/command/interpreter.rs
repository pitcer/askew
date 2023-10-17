use std::f32::consts;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::v2::curve::trochoid::TrochoidCurveProperties;
use crate::command::message::Message;
use crate::command::parser::{Command, Get, Set, Task, Toggle};
use crate::command::program_view::ProgramView;
use crate::config::CurveType;
use crate::event::canvas::{
    GetConvexHull, GetCurvesLength, GetLength, GetPointOnCurve, MoveCurve, MovePointOnCurve,
    RotateCurve, RotateCurveById, SetConvexHull, SetCurveType,
};
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::formula::SetTrochoidProperties;
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::{DelegateEventHandler, EventHandler};
use crate::ui::frame::event_handler::CommandEventHandler;

pub struct CommandInterpreter<'a> {
    state: ProgramView<'a>,
}

type InterpretResult<E = anyhow::Error> = Result<Option<Message>, E>;

impl<'a> CommandInterpreter<'a> {
    #[must_use]
    pub fn new(state: ProgramView<'a>) -> Self {
        Self { state }
    }

    fn command_handler(&mut self) -> CommandEventHandler<'_> {
        self.state.frame.event_handler(self.state.mode)
    }

    pub fn interpret(&mut self, command: Command) -> Result<Option<Message>, Error> {
        let result = match command {
            Command::Get(get) => self.interpret_get(get),
            Command::Set(set) => self.interpret_set(set),
            Command::Toggle(toggle) => self.interpret_toggle(toggle),
            Command::Rotate { angle, curve_id } => self.interpret_rotate(angle, curve_id),
            Command::Move { horizontal, vertical } => self.interpret_move(horizontal, vertical),
            Command::Save { path } => self.interpret_save(path),
            Command::Open { path } => self.interpret_open(path),
            Command::SaveImage { path } => self.interpret_save_image(path),
            Command::SetCurveType { curve_type } => self.interpret_set_curve_type(curve_type),
            Command::GetLength { curve_id } => self.get_length(curve_id),
            Command::GetPoint { curve_id, point_id } => self.get_point(curve_id, point_id),
            Command::MovePoint { curve_id, point_id, horizontal, vertical } => {
                self.move_point(curve_id, point_id, horizontal, vertical)
            }
            Command::GetCurvesLength => self.get_curves_length(),
            Command::TrochoidProperties(properties) => self.trochoid(properties),
            Command::Execute { path, argument } => self.execute(path, argument),
            Command::Task(task) => self.task(task),
            Command::Quit => self.quit(),
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
            Set::ConvexHull { value } => handler.delegate(SetConvexHull(value))?,
            Set::InterpolationNodes { value } => {
                handler.delegate(SetInterpolationNodes::new(value))?;
            }
            Set::Samples { value } => handler.delegate(SetSamples(value))?,
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
                // TODO: handle for current curve instead
                self.state.frame.canvas_mut().config.show_control_line =
                    !self.state.frame.canvas().config.show_control_line;
            }
        }
        Ok(None)
    }

    fn interpret_rotate(&mut self, angle: u16, curve: Option<usize>) -> InterpretResult {
        let mut handler = self.command_handler();
        let radians = consts::PI * f32::from(angle) / 180.0;
        if let Some(curve) = curve {
            handler.delegate(RotateCurveById::new(radians, curve))?;
        } else {
            handler.delegate(RotateCurve::new(radians))?;
        }
        Ok(Some(Message::info(format!("Curve rotated by {angle} deg"))))
    }

    fn interpret_move(&mut self, horizontal: f32, vertical: f32) -> InterpretResult {
        let mut handler = self.command_handler();
        let shift = Vector::new(horizontal, vertical);
        handler.delegate(MoveCurve::new(shift))?;
        Ok(Some(Message::info(format!("Curve moved by ({horizontal}, {vertical})"))))
    }

    fn interpret_save(&mut self, path: Option<PathBuf>) -> InterpretResult {
        let path = self.state.frame.save_canvas(path)?;
        let path = path.display();
        Ok(Some(Message::info(format!("Project saved into {path}"))))
    }

    fn interpret_open(&mut self, path: Option<PathBuf>) -> InterpretResult {
        let (path, canvas) = self.state.frame.open_canvas(path)?;
        let path = path.display();
        let message = Message::info(format!("Project opened from {path}"));
        self.state.frame.load_canvas(canvas);
        Ok(Some(message))
    }

    fn interpret_save_image(&mut self, path: Option<PathBuf>) -> InterpretResult {
        let path = self.state.frame.save_image(path)?;
        let path = path.display();
        Ok(Some(Message::info(format!("Canvas PNG image saved into '{path}'"))))
    }

    fn interpret_set_curve_type(&mut self, curve_type: CurveType) -> InterpretResult {
        self.command_handler().delegate(SetCurveType(curve_type))?;
        Ok(Some(Message::info(format!("Set curve type to {curve_type}"))))
    }

    fn get_curves_length(&mut self) -> InterpretResult {
        let result = self.command_handler().delegate(GetCurvesLength)?;
        Ok(Some(Message::info(format!("{result}"))))
    }

    fn get_length(&mut self, curve_id: usize) -> InterpretResult {
        let result = self.command_handler().delegate(GetLength(curve_id))?;
        Ok(Some(Message::info(format!("{result}"))))
    }

    fn get_point(&mut self, curve_id: usize, point_id: usize) -> InterpretResult {
        let result = self.command_handler().delegate(GetPointOnCurve(curve_id, point_id))?;
        Ok(Some(Message::info(format!("{},{}", result.horizontal(), result.vertical()))))
    }

    fn move_point(&mut self, curve_id: usize, point_id: usize, x: f32, y: f32) -> InterpretResult {
        self.command_handler().delegate(MovePointOnCurve(curve_id, point_id, Point::new(x, y)))?;
        Ok(None)
    }

    fn trochoid(&mut self, prop: TrochoidCurveProperties) -> InterpretResult {
        self.state
            .frame
            .canvas_mut()
            .current_curve_mut()
            .event_handler()
            .handle(SetTrochoidProperties(prop))?;
        Ok(None)
    }

    fn execute(&mut self, path: PathBuf, argument: Option<String>) -> InterpretResult {
        if !path.exists() {
            return Err(anyhow!("File '{}' does not exists", path.display()));
        }

        let task_id = self.state.tasks.register_task(path, argument)?;
        Ok(Some(Message::info(format!("Created task {task_id}"))))
    }

    fn task(&mut self, task: Task) -> InterpretResult {
        match task {
            Task::List => {
                let tasks =
                    self.state.tasks.list_tasks().map(|task| format!("({task})")).join(", ");
                Ok(Some(Message::info(format!("Tasks: {tasks}"))))
            }
            Task::Kill { task_id } => {
                if !self.state.tasks.task_exists(task_id) {
                    return Ok(Some(Message::error(format!("Task {task_id} does not exist"))));
                }

                self.state.tasks.kill_task(task_id);
                Ok(Some(Message::info(format!("Task {task_id} killed"))))
            }
        }
    }

    fn quit(&mut self) -> InterpretResult {
        self.state.control_flow.set_exit();
        Ok(Some(Message::info("Quit...".to_owned())))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown command")]
    UnknownCommand,
    #[error("other error: {0}")]
    OtherError(anyhow::Error),
}
