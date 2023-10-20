use std::mem;
use std::time::Duration;

use anyhow::{anyhow, Result};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::request::declare::RotateCurveById;
use crate::canvas::shape::request::declare::{GetCurveCenter, MoveCurve};
use crate::command;
use crate::command::program_view::ProgramView;
use crate::ipc::server::IpcServerHandle;
use crate::request::{RequestSubHandler, RequestSubHandlerMut};
use crate::ui::command_state::CommandState;
use crate::ui::frame::panel::Panel;
use crate::ui::frame::Frame;
use crate::ui::input_handler::Input;
use crate::ui::input_handler::InputHandler;
use crate::ui::painter::view::WindowView;
use crate::ui::painter::Painter;
use crate::ui::runner::request::{RunnerRequest, RunnerSender};
use crate::ui::runner::task::sleep::SleepingTasks;
use crate::ui::runner::task::Tasks;
use crate::ui::window::Window;
use crate::ui::window_handler::WindowEventHandler;
use crate::wasm::request::{Request, Response};
use crate::wasm::state::RequestHandle;

pub mod request;
pub mod task;

pub struct WindowRunner {
    commands: Vec<String>,
    window: Window,
    frame: Frame,
    painter: Painter,
    command: CommandState,
    event_handler: WindowEventHandler,
    ipc_server: Option<IpcServerHandle>,
    tasks: Tasks,
    sleeping_tasks: SleepingTasks,
}

impl WindowRunner {
    pub fn new(
        commands: Vec<String>,
        window: Window,
        frame: Frame,
        painter: Painter,
        ipc_server: IpcServerHandle,
        sender: RunnerSender,
    ) -> Result<Self> {
        let command = CommandState::new();
        let event_handler = WindowEventHandler::new();
        let ipc_server = Some(ipc_server);
        let tasks = Tasks::new(sender.clone())?;
        let sleeping_tasks = SleepingTasks::new();

        Ok(Self {
            commands,
            window,
            frame,
            painter,
            command,
            event_handler,
            ipc_server,
            tasks,
            sleeping_tasks,
        })
    }

    pub fn run(
        &mut self,
        event: Event<RunnerRequest>,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        log::debug!("<cyan><b>Event loop:</>\n<bright_black>{event:?}</>");

        match event {
            Event::RedrawRequested(window_id) if self.window.has_id(window_id) => {
                self.frame.canvas_mut().update_all();
                self.paint()?;
            }
            Event::WindowEvent { event, window_id } if self.window.has_id(window_id) => {
                let input = self.handle_window_event(event, control_flow)?;
                if let Some(input) = input {
                    let state = ProgramView::new(control_flow, &mut self.frame, &mut self.tasks);
                    let handler = InputHandler::new(&mut self.command, state);
                    let result = handler.handle_input(input);
                    if let Err(error) = result {
                        log::debug!("{error}");
                    }
                    self.window.request_redraw();
                }
            }
            Event::UserEvent(request) => self.handle_request(request, control_flow)?,
            Event::NewEvents(StartCause::Init) => {
                control_flow.set_wait();

                for command in mem::take(&mut self.commands) {
                    log::debug!("<cyan>Initial command input:</> '{command}'");

                    let state = ProgramView::new(control_flow, &mut self.frame, &mut self.tasks);
                    let result = command::execute(&command, state)?;

                    log::info!("Initial command result: `{result:?}`");
                }
            }
            Event::NewEvents(StartCause::ResumeTimeReached { start: _start, requested_resume }) => {
                let wake_time = self.sleeping_tasks.wake(requested_resume)?;
                match wake_time {
                    None => control_flow.set_wait(),
                    Some(wake_time) => control_flow.set_wait_until(wake_time),
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn paint(&mut self) -> Result<()> {
        let size = self.window.size_rectangle();
        let mut buffer = self.window.buffer_mut()?;
        let panel = Panel::from_buffer(&mut buffer, size);
        let view = WindowView::new(&self.frame, &self.command);

        self.painter.paint(view, panel)?;

        let result = buffer.present();
        result.map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }

    pub fn handle_window_event(
        &mut self,
        event: WindowEvent,
        control_flow: &mut ControlFlow,
    ) -> Result<Option<Input>> {
        match event {
            WindowEvent::Resized(size) => {
                self.window.resize_surface(size)?;
                Ok(None)
            }
            WindowEvent::CloseRequested => {
                if let Some(handle) = self.ipc_server.take() {
                    handle.close();
                }

                control_flow.set_exit();
                Ok(None)
            }
            _ => self.event_handler.handle(event),
        }
    }

    fn handle_request(
        &mut self,
        request: RunnerRequest,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        match request {
            RunnerRequest::IpcMessage(message) => {
                let state = ProgramView::new(control_flow, &mut self.frame, &mut self.tasks);
                let reply = message.handle(state);
                let handle = self.ipc_server.as_ref().expect("IPC server should exist");
                handle.send(reply)?;
                self.window.request_redraw();
                Ok(())
            }
            RunnerRequest::TaskRequest(request) => self.handle_task_request(request, control_flow),
            RunnerRequest::ProgressTask(task) => {
                let task_id = task.task_id();
                task.progress();

                if let Some(result) = self.tasks.try_finish_task(task_id) {
                    let result = result?;
                    log::info!("Task {task_id} finished with result: `{result:?}`");
                }
                Ok(())
            }
            RunnerRequest::ProgressIpcServer(runnable) => {
                runnable.run();
                Ok(())
            }
        }
    }

    fn handle_task_request(
        &mut self,
        request: RequestHandle,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        let RequestHandle { request, responder } = request;
        match request {
            Request::MoveCurve { id: _id, horizontal, vertical } => {
                // TODO: move curve specified by id
                let shift = Vector::new(horizontal, vertical);
                self.frame.sub_handle_mut(MoveCurve::new(shift))?;
                responder.respond(Response::Empty);
                self.window.request_redraw();
                Ok(())
            }
            Request::RotateCurve { id, angle_radians } => {
                self.frame.sub_handle_mut(RotateCurveById::new(angle_radians, id as usize))?;
                responder.respond(Response::Empty);
                self.window.request_redraw();
                Ok(())
            }
            Request::Sleep { seconds, nanoseconds } => {
                if seconds == 0 && nanoseconds == 0 {
                    responder.respond(Response::Sleep);
                    return Ok(());
                }

                let duration = Duration::new(seconds, nanoseconds);
                let wake_time = self.sleeping_tasks.sleep(responder, duration);
                control_flow.set_wait_until(wake_time);
                Ok(())
            }
            Request::GetPosition { id: _id } => {
                // TODO: get position of curve specified by id
                let center = self.frame.sub_handle(GetCurveCenter)?;
                // TODO: return None instead of (0, 0)
                let center = center.unwrap_or_else(|| Point::new(0.0, 0.0));
                responder.respond(Response::GetPosition {
                    horizontal: center.horizontal(),
                    vertical: center.vertical(),
                });
                Ok(())
            }
        }
    }
}
