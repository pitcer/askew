use std::mem;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use async_io::Timer;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;

use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::request::declare::RotateCurveById;
use crate::canvas::shape::request::declare::{GetCurveCenter, MoveCurve};
use crate::command::program_view::ProgramView;
use crate::ipc::server::IpcServerHandle;
use crate::request::{RequestSubHandler, RequestSubHandlerMut};
use crate::ui::command_state::CommandState;
use crate::ui::frame::panel::Panel;
use crate::ui::frame::Frame;
use crate::ui::handler::message::{HandlerMessage, RunnerSender};
use crate::ui::input_event_handler::InputEventHandler;
use crate::ui::input_handler::InputHandler;
use crate::ui::painter::view::WindowView;
use crate::ui::painter::Painter;
use crate::ui::task::Tasks;
use crate::ui::window;
use crate::ui::window::Window;
use crate::wasm::request::{Request, Response};
use crate::wasm::state::RequestHandle;
use crate::{command, executor};

pub mod message;

pub struct WindowHandler<'a> {
    commands: Vec<String>,
    window: Window<'a>,
    frame: Frame,
    painter: Painter,
    command: CommandState,
    event_handler: InputEventHandler,
    ipc_server: Option<IpcServerHandle>,
    tasks: Tasks,
}

impl<'a> WindowHandler<'a> {
    pub fn new(
        commands: Vec<String>,
        window: Window<'a>,
        frame: Frame,
        painter: Painter,
        ipc_server: IpcServerHandle,
        sender: RunnerSender,
    ) -> Result<WindowHandler<'a>> {
        let command = CommandState::new();
        let event_handler = InputEventHandler::new();
        let ipc_server = Some(ipc_server);
        let tasks = Tasks::new(sender.clone())?;

        Ok(Self { commands, window, frame, painter, command, event_handler, ipc_server, tasks })
    }

    pub fn handle(
        &mut self,
        event: Event<HandlerMessage>,
        target: &EventLoopWindowTarget<HandlerMessage>,
    ) {
        log::debug!("<cyan><b>Event loop:</>\n<bright_black>{event:?}</>");
        let result = self.handle_event(event, target);
        if let Err(error) = result {
            log::error!("Error during event handling: {error}");
        }
    }

    fn handle_event(
        &mut self,
        event: Event<HandlerMessage>,
        target: &EventLoopWindowTarget<HandlerMessage>,
    ) -> Result<()> {
        match event {
            Event::WindowEvent { event, window_id } if self.window.has_id(window_id) => {
                self.handle_window_event(event, target)?;
            }
            Event::UserEvent(request) => self.handle_request(request, target)?,
            Event::NewEvents(StartCause::Init) => {
                for command in mem::take(&mut self.commands) {
                    log::debug!("<cyan>Initial command input:</> '{command}'");

                    let state = ProgramView::new(target, &mut self.frame, &mut self.tasks);
                    let result = command::execute(&command, state)?;

                    log::info!("Initial command result: `{result:?}`");
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent,
        target: &EventLoopWindowTarget<HandlerMessage>,
    ) -> Result<()> {
        match event {
            WindowEvent::RedrawRequested => {
                self.paint()?;
            }
            WindowEvent::Resized(size) => {
                self.window.resize_surface(size)?;
            }
            WindowEvent::CloseRequested => {
                if let Some(handle) = self.ipc_server.take() {
                    handle.close();
                }

                target.exit();
            }
            _ => {
                let input = self.event_handler.handle(event)?;
                if let Some(input) = input {
                    let state = ProgramView::new(target, &mut self.frame, &mut self.tasks);
                    let handler = InputHandler::new(&mut self.command, state);
                    let result = handler.handle_input(input);
                    if let Err(error) = result {
                        log::debug!("{error}");
                    }
                    self.window.request_redraw();
                }
            }
        }
        Ok(())
    }

    fn paint(&mut self) -> Result<()> {
        let size = self.window.size_rectangle();
        let mut buffer = self.window.buffer_mut()?;
        let pixels = window::buffer_as_pixels(&mut buffer);
        let panel = Panel::new(pixels, size);
        let view = WindowView::new(&self.frame, &self.command);

        self.painter.paint(view, panel)?;

        let result = buffer.present();
        result.map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }

    fn handle_request(
        &mut self,
        request: HandlerMessage,
        target: &EventLoopWindowTarget<HandlerMessage>,
    ) -> Result<()> {
        match request {
            HandlerMessage::IpcMessage(message) => {
                let state = ProgramView::new(target, &mut self.frame, &mut self.tasks);
                let reply = message.handle(state);
                let handle = self.ipc_server.as_ref().expect("IPC server should exist");
                handle.send(reply)?;
                self.window.request_redraw();
                Ok(())
            }
            HandlerMessage::TaskRequest(request) => self.handle_task_request(request),
            HandlerMessage::TaskFinished(task_id) => {
                let result = self.tasks.finish_task(task_id);
                let result = result?;
                log::info!("Task {task_id} finished with result: `{result:?}`");
                Ok(())
            }
        }
    }

    fn handle_task_request(&mut self, request: RequestHandle) -> Result<()> {
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
                let timer = Timer::after(duration);
                let timer_future = async move {
                    timer.await;
                    responder.respond(Response::Sleep);
                };
                let task = executor::spawn(timer_future);
                // TODO: Save task in vec or something that will allow to list or kill it
                task.detach();
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
