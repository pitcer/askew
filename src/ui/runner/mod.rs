use std::time::Duration;

use anyhow::{anyhow, Result};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::command::interpreter::CommandInterpreter;
use crate::command::parser::CommandParser;
use crate::command::program_view::ProgramView;
use crate::command::CommandState;
use crate::event::canvas::RotateCurveById;
use crate::event::DelegateEventHandler;
use crate::ipc::server::IpcServerHandle;
use crate::ui::frame::panel::Panel;
use crate::ui::frame::Frame;
use crate::ui::input_handler::Input;
use crate::ui::input_handler::InputHandler;
use crate::ui::mode::ModeState;
use crate::ui::painter::view::WindowView;
use crate::ui::painter::Painter;
use crate::ui::runner::task::Tasks;
use crate::ui::runner::window_request::{EventLoopRequest, EventLoopSender};
use crate::ui::window::Window;
use crate::ui::window_handler::WindowEventHandler;
use crate::wasm::request::{Request, Response};
use crate::wasm::RequestHandle;

pub mod task;
pub mod waker;
pub mod window_request;

pub struct WindowRunner {
    window: Window,
    frame: Frame,
    painter: Painter,
    command: CommandState,
    mode: ModeState,
    event_handler: WindowEventHandler,
    ipc_server: Option<IpcServerHandle>,
    tasks: Tasks,
}

impl WindowRunner {
    pub fn new(
        window: Window,
        frame: Frame,
        painter: Painter,
        handle: IpcServerHandle,
        sender: EventLoopSender,
    ) -> Result<Self> {
        let command = CommandState::new();
        let mode = ModeState::new();
        let event_handler = WindowEventHandler::new();
        let ipc_server = Some(handle);
        let tasks = Tasks::new(sender.clone())?;

        Ok(Self {
            window,
            frame,
            painter,
            command,
            mode,
            event_handler,
            ipc_server,
            tasks,
        })
    }

    pub fn run(
        &mut self,
        event: Event<EventLoopRequest>,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        match event {
            Event::RedrawRequested(window_id) if self.window.has_id(window_id) => {
                self.paint()?;
            }
            Event::WindowEvent { event, window_id } if self.window.has_id(window_id) => {
                let input = self.handle_window_event(event, control_flow)?;
                if let Some(input) = input {
                    let state = ProgramView::new(&mut self.mode, &mut self.frame, &mut self.tasks);
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
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                // TODO: wake task from sleep
                control_flow.set_wait();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn paint(&mut self) -> Result<()> {
        let size = self.window.size_rectangle();
        let mut buffer = self.window.buffer_mut()?;
        let panel = Panel::from_buffer(&mut buffer, size);
        let view = WindowView::new(&self.frame, &self.command, &self.mode);

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
                self.frame.handle_close()?;

                let handle = self.ipc_server.take().expect("IPC server should exist");
                handle.close();

                control_flow.set_exit();
                Ok(None)
            }
            _ => self.event_handler.handle(event),
        }
    }

    fn handle_request(
        &mut self,
        request: EventLoopRequest,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        match request {
            EventLoopRequest::NoReplyCommand(command) => {
                let state = ProgramView::new(&mut self.mode, &mut self.frame, &mut self.tasks);
                let mut parser = CommandParser::new(&command);
                let result = parser.parse()?;
                let mut interpreter = CommandInterpreter::new(state);
                let _ = interpreter.interpret(result)?;
                Ok(())
            }
            EventLoopRequest::IpcMessage(message) => {
                let state = ProgramView::new(&mut self.mode, &mut self.frame, &mut self.tasks);
                let reply = message.handle(state);
                let handle = self.ipc_server.as_ref().expect("IPC server should exist");
                handle.send(reply)?;
                self.window.request_redraw();
                Ok(())
            }
            EventLoopRequest::TaskRequest(request) => {
                self.handle_task_request(request, control_flow)
            }
            EventLoopRequest::ProgressTask(task) => {
                let task_id = task.task_id();
                task.progress();

                if let Some(result) = self.tasks.try_finish_task(task_id) {
                    log::debug!("task result: {result:?}");
                }
                Ok(())
            }
            EventLoopRequest::ProgressIpcServer(runnable) => {
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
        let RequestHandle {
            request,
            response_sender,
        } = request;
        match request {
            Request::RotateCurve { id, angle } => {
                self.frame
                    .event_handler(&mut self.mode)
                    .delegate(RotateCurveById::new(angle, id))?;
                response_sender.send_blocking(Response::Empty)?;
                self.window.request_redraw();
                Ok(())
            }
            Request::Sleep { seconds } => {
                // TODO: register that this task is sleeping
                control_flow.set_wait_timeout(Duration::from_secs(seconds));
                response_sender.send_blocking(Response::Empty)?;
                Ok(())
            }
        }
    }
}