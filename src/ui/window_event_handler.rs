use anyhow::Result;
use input::{
    ChangeIndex, ChangeMode, ChangeWeight, Delete, EnterCommand, ExecuteCommand, ExitMode,
    MouseClick, MovePoint, ReceiveCharacter, ToggleConvexHull,
};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    DeviceId, ElementState, Event, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
    WindowEvent,
};
use winit::event_loop::ControlFlow;

use crate::config::{Config, SaveFormat};
use crate::event::{canvas, input, Change, Direction, InputEvent};
use crate::ui::frame::mode::Mode;
use crate::ui::frame::Frame;

pub struct WindowEventHandler {
    frame: Frame,
    cursor_position: PhysicalPosition<f64>,
    modifiers: ModifiersState,
    save_format: Option<SaveFormat>,
}

impl WindowEventHandler {
    #[must_use]
    pub fn new(frame: Frame, config: &Config) -> Self {
        let cursor_position = PhysicalPosition::new(0.0, 0.0);
        let modifiers = ModifiersState::empty();
        Self {
            frame,
            cursor_position,
            modifiers,
            save_format: config.save_format,
        }
    }

    pub fn run(&mut self, event: Event<'_, ()>, control_flow: &mut ControlFlow) -> Result<()> {
        control_flow.set_wait();

        match event {
            Event::RedrawRequested(window_id) if self.frame.has_id(window_id) => {
                self.frame.draw()?;
            }
            Event::WindowEvent { event, window_id } if self.frame.has_id(window_id) => {
                let event = self.handle_window_event(event, control_flow)?;
                if let Some(event) = event {
                    self.frame.receive_event(event)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent<'_>,
        control_flow: &mut ControlFlow,
    ) -> Result<Option<InputEvent>> {
        match event {
            WindowEvent::Resized(size) => {
                self.handle_resized(size)?;
            }
            WindowEvent::CloseRequested => {
                if let Some(format) = self.save_format {
                    self.frame.save(format)?;
                }
                control_flow.set_exit();
            }
            WindowEvent::ReceivedCharacter(character) => {
                return Ok(Some(InputEvent::ReceiveCharacter(ReceiveCharacter(
                    character,
                ))));
            }
            WindowEvent::KeyboardInput {
                device_id, input, ..
            } => return Ok(self.handle_keyboard_input(device_id, input)),
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers;
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
            } => return Ok(self.handle_mouse_input(device_id, state, button)),
            _ => {}
        }
        Ok(None)
    }

    fn handle_resized(&mut self, size: PhysicalSize<u32>) -> Result<()> {
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
    ) -> Option<InputEvent> {
        if state == ElementState::Pressed && button == MouseButton::Left {
            return Some(InputEvent::MouseClick(MouseClick(self.cursor_position)));
        }
        None
    }

    fn handle_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        input: KeyboardInput,
    ) -> Option<InputEvent> {
        log::debug!("keyboard_input: {input:?}");

        if input.state != ElementState::Pressed {
            return None;
        }

        match input.virtual_keycode {
            Some(VirtualKeyCode::Colon) => Some(InputEvent::EnterCommand(EnterCommand)),
            Some(VirtualKeyCode::Return) => Some(InputEvent::ExecuteCommand(ExecuteCommand)),
            Some(VirtualKeyCode::Escape) => Some(InputEvent::ExitMode(ExitMode)),

            Some(VirtualKeyCode::C) => Some(InputEvent::ChangeMode(ChangeMode(Mode::Point))),
            Some(VirtualKeyCode::A) => Some(InputEvent::AddCurve(canvas::AddCurve)),
            Some(VirtualKeyCode::D) => Some(InputEvent::Delete(Delete)),

            Some(VirtualKeyCode::J) => Some(InputEvent::ChangeIndex(ChangeIndex(Change::Decrease))),
            Some(VirtualKeyCode::K) => Some(InputEvent::ChangeIndex(ChangeIndex(Change::Increase))),

            Some(VirtualKeyCode::I) => {
                Some(InputEvent::ChangeWeight(ChangeWeight(Change::Increase)))
            }
            Some(VirtualKeyCode::O) => {
                Some(InputEvent::ChangeWeight(ChangeWeight(Change::Decrease)))
            }

            Some(VirtualKeyCode::H) => Some(InputEvent::ToggleConvexHull(ToggleConvexHull)),

            Some(VirtualKeyCode::Up) => Some(InputEvent::MovePoint(MovePoint(Direction::Up))),
            Some(VirtualKeyCode::Down) => Some(InputEvent::MovePoint(MovePoint(Direction::Down))),
            Some(VirtualKeyCode::Left) => Some(InputEvent::MovePoint(MovePoint(Direction::Left))),
            Some(VirtualKeyCode::Right) => Some(InputEvent::MovePoint(MovePoint(Direction::Right))),
            _ => None,
        }
    }
}
