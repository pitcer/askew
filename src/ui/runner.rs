use anyhow::{anyhow, Result};
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::ipc::server::{IpcMessage, IpcServerHandle};
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

pub struct WindowRunner {
    window: Window,
    frame: Frame,
    painter: Painter,
    command: CommandState,
    mode: ModeState,
    event_handler: WindowEventHandler,
    handle: Option<IpcServerHandle>,
}

impl WindowRunner {
    #[must_use]
    pub fn new(window: Window, frame: Frame, painter: Painter, handle: IpcServerHandle) -> Self {
        let command = CommandState::initial();
        let mode = ModeState::initial();
        let event_handler = WindowEventHandler::new();
        Self {
            window,
            frame,
            painter,
            command,
            mode,
            event_handler,
            handle: Some(handle),
        }
    }

    pub fn run(
        &mut self,
        event: Event<'_, IpcMessage>,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        control_flow.set_wait();

        match event {
            Event::RedrawRequested(window_id) if self.window.has_id(window_id) => {
                self.paint()?;
            }
            Event::WindowEvent { event, window_id } if self.window.has_id(window_id) => {
                let event = self.handle_window_event(event, control_flow)?;
                if let Some(event) = event {
                    let state = ProgramState::new(&mut self.mode, &mut self.frame);
                    let handler = InputHandler::new(&mut self.command, state);
                    let result = handler.handle_input(event);
                    if let Err(error) = result {
                        log::debug!("{error}");
                    }
                    self.window.request_redraw();
                }
            }
            Event::UserEvent(message) => {
                let state = ProgramState::new(&mut self.mode, &mut self.frame);
                let reply = message.handle(state);
                let handle = self.handle.as_ref().expect("handle should exist");
                handle.send(reply)?;
                self.window.request_redraw();
            }
            Event::LoopDestroyed => {
                let handle = self.handle.take().expect("handle should exist");
                handle.close()?;
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
                control_flow.set_exit();
                Ok(None)
            }
            _ => self.event_handler.handle(event),
        }
    }
}
