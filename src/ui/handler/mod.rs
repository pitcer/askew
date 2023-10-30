use anyhow::anyhow;
use anyhow::Result;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;

use crate::command;
use crate::command::program_view::ProgramView;
use crate::ui::frame::panel::Panel;
use crate::ui::handler::input_event::InputEventHandler;
use crate::ui::handler::message::{HandlerMessage, HandlerSender};
use crate::ui::input_handler::{Input, InputHandler};
use crate::ui::painter::view::WindowView;
use crate::ui::painter::Painter;
use crate::ui::shared::SharedState;
use crate::ui::window;
use crate::ui::window::Window;

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
        painter: Painter,
        sender: HandlerSender,
        state: SharedState,
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
            let (mut frame, mut tasks) = self.state.lock_blocking();
            let state = ProgramView::new(sender, &mut frame, &mut tasks);
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
        let frame = self.state.frame().lock_blocking();
        let view = WindowView::new(&frame, self.input_handler.command());

        self.painter.paint(view, panel)?;

        let result = buffer.present();
        result.map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }

    fn handle_input(&mut self, input: Input) -> Result<()> {
        let sender = HandlerSender::clone(&self.sender);
        let (mut frame, mut tasks) = self.state.lock_blocking();
        let state = ProgramView::new(sender, &mut frame, &mut tasks);
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
            HandlerMessage::TaskFinished(task_id, result) => {
                log::info!("Task {task_id} finished with result: `{result:?}`");
                let (_, mut tasks) = self.state.lock_blocking();
                tasks.finish_task(task_id)?;
            }
            HandlerMessage::TaskYield(response) => response.send(),
            HandlerMessage::Redraw => self.window.request_redraw(),
            HandlerMessage::Exit => target.exit(),
        }
        Ok(())
    }
}
