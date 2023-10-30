use anyhow::anyhow;
use anyhow::Result;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;

use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::request::declare::RotateCurveById;
use crate::canvas::shape::request::declare::{GetCurveCenter, MoveCurve};
use crate::command;
use crate::command::program_view::ProgramView;
use crate::request::{RequestSubHandler, RequestSubHandlerMut};
use crate::ui::frame::panel::Panel;
use crate::ui::handler::input_event::InputEventHandler;
use crate::ui::handler::message::{HandlerMessage, HandlerSender};
use crate::ui::input_handler::{Input, InputHandler};
use crate::ui::painter::view::WindowView;
use crate::ui::painter::Painter;
use crate::ui::state::SharedState;
use crate::ui::window;
use crate::ui::window::Window;
use crate::wasm::request::{Request, Response};
use crate::wasm::state::RequestHandle;

pub mod input_event;
pub mod message;

pub struct WindowHandler<'a> {
    sender: HandlerSender,
    window: Window<'a>,
    painter: Painter,
    event_handler: InputEventHandler,
    input_handler: InputHandler,
    state: SharedState,
}

impl<'a> WindowHandler<'a> {
    pub fn new(
        commands: Vec<String>,
        window: Window<'a>,
        state: SharedState,
        painter: Painter,
        sender: HandlerSender,
    ) -> Result<WindowHandler<'a>> {
        let event_handler = InputEventHandler::new();
        let input_handler = InputHandler::new();

        let mut window_handler =
            Self { sender, window, painter, event_handler, input_handler, state };
        window_handler.run_startup_commands(commands)?;
        Ok(window_handler)
    }

    fn run_startup_commands(&mut self, commands: Vec<String>) -> Result<()> {
        for command in commands {
            log::debug!("<cyan>Startup command input:</> '{command}'");

            let sender = HandlerSender::clone(&self.sender);
            let mut state = self.state.lock_arc_blocking();
            let state = ProgramView::new(sender, &mut state);
            let result = command::execute(&command, state)?;

            log::info!("Startup command result: `{result:?}`");
        }
        Ok(())
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
            _ => (),
        }
        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent,
        target: &EventLoopWindowTarget<HandlerMessage>,
    ) -> Result<()> {
        match event {
            WindowEvent::RedrawRequested => self.paint()?,
            WindowEvent::Resized(size) => self.window.resize_surface(size)?,
            WindowEvent::CloseRequested => target.exit(),
            _ => {
                let input = self.event_handler.handle(event)?;
                if let Some(input) = input {
                    self.handle_input(input)?;
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
        let state = self.state.lock_arc_blocking();
        let view = WindowView::new(&state.frame, self.input_handler.command());

        self.painter.paint(view, panel)?;

        let result = buffer.present();
        result.map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }

    fn handle_input(&mut self, input: Input) -> Result<()> {
        let sender = HandlerSender::clone(&self.sender);
        let mut state = self.state.lock_arc_blocking();
        let state = ProgramView::new(sender, &mut state);
        let result = self.input_handler.handle_input(input, state);
        if let Err(error) = result {
            log::error!("Error during handling input: `{error}`");
        }
        self.sender.send_event(HandlerMessage::Redraw)?;
        Ok(())
    }

    fn handle_request(
        &mut self,
        request: HandlerMessage,
        target: &EventLoopWindowTarget<HandlerMessage>,
    ) -> Result<()> {
        match request {
            HandlerMessage::TaskRequest(request) => self.handle_task_request(request)?,
            HandlerMessage::TaskFinished(task_id, result) => {
                log::info!("Task {task_id} finished with result: `{result:?}`");
            }
            HandlerMessage::Redraw => self.window.request_redraw(),
            HandlerMessage::Exit => target.exit(),
        }
        Ok(())
    }

    fn handle_task_request(&mut self, request: RequestHandle) -> Result<()> {
        let RequestHandle { request, responder } = request;
        match request {
            Request::MoveCurve { id: _id, horizontal, vertical } => {
                // TODO: move curve specified by id
                let shift = Vector::new(horizontal, vertical);
                self.state.lock_arc_blocking().frame.sub_handle_mut(MoveCurve::new(shift))?;
                responder.respond(Response::Empty);
                self.window.request_redraw();
                Ok(())
            }
            Request::RotateCurve { id, angle_radians } => {
                self.state
                    .lock_arc_blocking()
                    .frame
                    .sub_handle_mut(RotateCurveById::new(angle_radians, id as usize))?;
                responder.respond(Response::Empty);
                self.window.request_redraw();
                Ok(())
            }
            Request::GetPosition { id: _id } => {
                // TODO: get position of curve specified by id
                let center = self.state.lock_blocking().frame.sub_handle(GetCurveCenter)?;
                // TODO: return None instead of (0, 0)
                let center = center.unwrap_or_else(|| Point::new(0.0, 0.0));
                responder.respond(Response::GetPosition {
                    horizontal: center.horizontal(),
                    vertical: center.vertical(),
                });
                Ok(())
            }
            Request::Yield => {
                responder.respond(Response::Yield);
                Ok(())
            }
        }
    }
}
