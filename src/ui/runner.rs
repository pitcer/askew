use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use async_task::Task;
use futures_lite::future;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::event::canvas::RotateCurveById;
use crate::event::DelegateEventHandler;
use crate::ipc::server::IpcServerHandle;
use crate::ui::command::interpreter::CommandInterpreter;
use crate::ui::command::parser::CommandParser;
use crate::ui::command::CommandState;
use crate::ui::frame::panel::Panel;
use crate::ui::frame::Frame;
use crate::ui::input_handler::InputEvent;
use crate::ui::input_handler::InputHandler;
use crate::ui::mode::ModeState;
use crate::ui::painter::view::WindowView;
use crate::ui::painter::Painter;
use crate::ui::state::ProgramState;
use crate::ui::window::Window;
use crate::ui::window_handler::WindowEventHandler;
use crate::wasm::request::{Request, Response};
use crate::wasm::WasmRuntime;
use crate::window_request::{EventLoopProxy, WindowRequest};

pub struct WindowRunner {
    window: Window,
    frame: Frame,
    painter: Painter,
    command: CommandState,
    mode: ModeState,
    event_handler: WindowEventHandler,
    handle: Option<IpcServerHandle>,
    proxy: EventLoopProxy,
    request_sender: Sender<Request>,
    _request_receiver: Receiver<Request>,
    response_sender: Sender<Response>,
    response_receiver: Receiver<Response>,
    wasm_runtime: Arc<WasmRuntime>,
    current_task: Option<Task<Result<u32>>>,
}

impl WindowRunner {
    pub fn new(
        window: Window,
        frame: Frame,
        painter: Painter,
        handle: IpcServerHandle,
        proxy: EventLoopProxy,
    ) -> Result<Self> {
        let command = CommandState::initial();
        let mode = ModeState::initial();
        let event_handler = WindowEventHandler::new();
        let (request_sender, request_receiver) = async_channel::unbounded();
        let (response_sender, response_receiver) = async_channel::unbounded();
        Ok(Self {
            window,
            frame,
            painter,
            command,
            mode,
            event_handler,
            handle: Some(handle),
            proxy,
            request_sender,
            _request_receiver: request_receiver,
            response_sender,
            response_receiver,
            wasm_runtime: Arc::new(WasmRuntime::new()?),
            current_task: None,
        })
    }

    pub fn run(
        &mut self,
        event: Event<'_, WindowRequest>,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        match event {
            Event::RedrawRequested(window_id) if self.window.has_id(window_id) => {
                self.paint()?;
            }
            Event::WindowEvent { event, window_id } if self.window.has_id(window_id) => {
                let event = self.handle_window_event(event, control_flow)?;
                if let Some(event) = event {
                    let state = ProgramState::new(&mut self.mode, &mut self.frame, &self.proxy);
                    let handler = InputHandler::new(&mut self.command, state);
                    let result = handler.handle_input(event);
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
                self.response_sender.send_blocking(Response::Empty)?;
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

        buffer
            .present()
            .map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }

    pub fn handle_window_event(
        &mut self,
        event: WindowEvent<'_>,
        control_flow: &mut ControlFlow,
    ) -> Result<Option<InputEvent>> {
        match event {
            WindowEvent::Resized(size) => {
                self.window.resize_surface(size)?;
                Ok(None)
            }
            WindowEvent::CloseRequested => {
                self.frame.handle_close()?;

                let handle = self.handle.take().expect("handle should exist");
                handle.close();

                control_flow.set_exit();
                Ok(None)
            }
            _ => self.event_handler.handle(event),
        }
    }

    fn handle_request(
        &mut self,
        request: WindowRequest,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        match request {
            WindowRequest::NoReplyCommand(command) => {
                let state = ProgramState::new(&mut self.mode, &mut self.frame, &self.proxy);
                let mut parser = CommandParser::new(&command);
                let result = parser.parse()?;
                let mut interpreter = CommandInterpreter::new(state);
                let _ = interpreter.interpret(result)?;
                Ok(())
            }
            WindowRequest::IpcMessage(message) => {
                let state = ProgramState::new(&mut self.mode, &mut self.frame, &self.proxy);
                let reply = message.handle(state);
                let handle = self.handle.as_ref().expect("handle should exist");
                handle.send(reply)?;
                self.window.request_redraw();
                Ok(())
            }
            WindowRequest::WasmRequest(request) => match request {
                Request::RotateCurve { id, angle } => {
                    self.frame
                        .event_handler(&mut self.mode)
                        .delegate(RotateCurveById::new(angle, id))?;
                    self.response_sender.send_blocking(Response::Empty)?;
                    self.window.request_redraw();
                    Ok(())
                }
                Request::Sleep { seconds } => {
                    control_flow.set_wait_timeout(Duration::from_secs(seconds));
                    Ok(())
                }
            },
            WindowRequest::WasmRun { path } => {
                let runtime = Arc::clone(&self.wasm_runtime);
                let proxy = self.proxy.clone();
                let sender = self.request_sender.clone();
                let receiver = self.response_receiver.clone();
                let future = async move { runtime.run(path, proxy, sender, receiver).await };

                let proxy = self.proxy.clone();
                let schedule =
                    move |runnable| proxy.send_event(WindowRequest::Progress(runnable)).unwrap();

                let (runnable, task) = async_task::spawn(future, schedule);

                self.current_task = Some(task);
                runnable.schedule();

                Ok(())
            }
            WindowRequest::Progress(runnable) => {
                runnable.run();

                if self.current_task.as_ref().unwrap().is_finished() {
                    let result = future::block_on(self.current_task.take().unwrap())?;
                    log::debug!("task result: {result}");
                }
                Ok(())
            }
            WindowRequest::ProgressIpcServer(runnable) => {
                runnable.run();
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub sender: Sender<WindowRequest>,
    pub receiver: Receiver<Response>,
}
