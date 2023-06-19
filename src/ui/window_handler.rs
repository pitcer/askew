use anyhow::Result;
use winit::dpi::PhysicalPosition;
use winit::event::{
    DeviceId, ElementState, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode, WindowEvent,
};

use crate::event::input::{
    ChangeIndex, ChangeWeight, Delete, MouseClick, MovePoint, ToggleConvexHull,
};
use crate::event::{canvas, Change, Direction};
use crate::ui::input_handler::InputEvent;
use crate::ui::mode::Mode;

pub struct WindowEventHandler {
    cursor_position: PhysicalPosition<f64>,
    modifiers: ModifiersState,
}

impl WindowEventHandler {
    #[must_use]
    pub fn new() -> Self {
        let cursor_position = PhysicalPosition::new(0.0, 0.0);
        let modifiers = ModifiersState::empty();
        Self {
            cursor_position,
            modifiers,
        }
    }

    pub fn handle(&mut self, event: WindowEvent<'_>) -> Result<Option<InputEvent>> {
        match event {
            WindowEvent::ReceivedCharacter(character) => {
                return Ok(Some(InputEvent::ReceiveCharacter(character)));
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
            Some(VirtualKeyCode::Colon) => Some(InputEvent::EnterCommand),
            Some(VirtualKeyCode::Return) => Some(InputEvent::ExecuteCommand),
            Some(VirtualKeyCode::Escape) => Some(InputEvent::ExitMode),

            Some(VirtualKeyCode::C) => Some(InputEvent::ChangeMode(Mode::Point)),
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

impl Default for WindowEventHandler {
    fn default() -> Self {
        Self::new()
    }
}
