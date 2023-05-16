use std::cell::RefCell;
use std::rc::Rc;

use crate::canvas::event::CanvasEvent;
use anyhow::Result;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    DeviceId, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};
use winit::event_loop::ControlFlow;

use crate::command::{Command, SaveFormat};
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
                self.frame.draw(None)?;
            }
            Event::WindowEvent { event, window_id } if self.frame.has_id(window_id) => {
                let event = self.handle_window_event(event, control_flow)?;
                self.frame.draw(event)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent,
        control_flow: &mut ControlFlow,
    ) -> Result<Option<CanvasEvent>> {
        match event {
            WindowEvent::Resized(size) => self.handle_resized(size),
            WindowEvent::CloseRequested => {
                if let Some(format) = self.save_format {
                    self.frame.save(format)?;
                }
                control_flow.set_exit()
            }
            WindowEvent::KeyboardInput {
                device_id, input, ..
            } => return Ok(self.handle_keyboard_input(device_id, input)),
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
        Ok(None)
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

    fn handle_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        input: KeyboardInput,
    ) -> Option<CanvasEvent> {
        if input.state != ElementState::Pressed {
            return None;
        }
        match input.virtual_keycode {
            Some(VirtualKeyCode::J) => Some(CanvasEvent::ChangeCurrentIndex(-1)),
            Some(VirtualKeyCode::K) => Some(CanvasEvent::ChangeCurrentIndex(1)),
            Some(VirtualKeyCode::I) => Some(CanvasEvent::ChangeWeight(1.5)),
            Some(VirtualKeyCode::O) => Some(CanvasEvent::ChangeWeight(-1.5)),
            Some(VirtualKeyCode::H) => Some(CanvasEvent::ToggleConvexHull),
            _ => None,
        }
    }
}
