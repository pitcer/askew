use crate::command::{Command, SaveFormat};
use anyhow::Result;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceId, ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::ui::frame::Frame;

pub struct EventHandler {
    frame: Frame,
    cursor_position: PhysicalPosition<f64>,
    save_format: Option<SaveFormat>,
}

impl EventHandler {
    pub fn new(frame: Frame, command: &Command) -> Self {
        let cursor_position = PhysicalPosition::new(0.0, 0.0);
        Self {
            frame,
            cursor_position,
            save_format: command.save_format,
        }
    }

    pub fn run(&mut self, event: Event<()>, control_flow: &mut ControlFlow) -> Result<()> {
        control_flow.set_wait();

        match event {
            Event::RedrawRequested(window_id) if self.frame.has_id(window_id) => {
                self.frame.draw()?;
            }
            Event::WindowEvent { event, window_id } if self.frame.has_id(window_id) => {
                self.handle_window_event(event, control_flow)?
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent,
        control_flow: &mut ControlFlow,
    ) -> Result<()> {
        match event {
            WindowEvent::Resized(size) => self.handle_resized(size),
            WindowEvent::CloseRequested => {
                if let Some(format) = self.save_format {
                    self.frame.save(format)?;
                }
                control_flow.set_exit()
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => self.handle_cursor_moved(device_id, position),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => self.handle_mouse_input(device_id, state, button),
            _ => {}
        }
        Ok(())
    }

    fn handle_resized(&mut self, size: PhysicalSize<u32>) {
        self.frame.resize(size)
    }

    fn handle_cursor_moved(&mut self, _device_id: DeviceId, position: PhysicalPosition<f64>) {
        self.cursor_position = position;
    }

    fn handle_mouse_input(
        &mut self,
        _device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    ) {
        if state == ElementState::Pressed && button == MouseButton::Left {
            self.frame.add_point(self.cursor_position)
        }
    }
}
