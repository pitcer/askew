use anyhow::Result;
use winit::dpi::PhysicalPosition;
use winit::event::{DeviceId, ElementState, KeyEvent, Modifiers, MouseButton, WindowEvent};
use winit::keyboard::Key;

use crate::request::{Change, Direction};
use crate::ui::frame::request::declare::{
    Add, ChangeIndex, ChangeWeight, Delete, MouseClick, MousePress, MovePoint, ToggleConvexHull,
};
use crate::ui::input_handler::{Input, InputEvent};
use crate::ui::mode::Mode;

pub struct WindowEventHandler {
    cursor_position: PhysicalPosition<f64>,
    modifiers: Modifiers,
    mouse_left_state: ElementState,
}

impl WindowEventHandler {
    #[must_use]
    pub fn new() -> Self {
        let cursor_position = PhysicalPosition::new(0.0, 0.0);
        let modifiers = Modifiers::default();
        let mouse_left_state = ElementState::Released;
        Self { cursor_position, modifiers, mouse_left_state }
    }

    pub fn handle(&mut self, event: WindowEvent) -> Result<Option<Input>> {
        match event {
            WindowEvent::KeyboardInput { device_id, event, .. } => {
                return Ok(self.handle_keyboard_input(device_id, event))
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers;
            }
            WindowEvent::CursorMoved { device_id, position, .. } => {
                return Ok(self
                    .handle_cursor_moved(device_id, position)
                    .map(|event| Input::new(Some(event), None)))
            }
            WindowEvent::MouseInput { device_id, state, button, .. } => {
                return Ok(self
                    .handle_mouse_input(device_id, state, button)
                    .map(|event| Input::new(Some(event), None)))
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_cursor_moved(
        &mut self,
        _device_id: DeviceId,
        position: PhysicalPosition<f64>,
    ) -> Option<InputEvent> {
        self.cursor_position = position;

        if self.mouse_left_state == ElementState::Pressed {
            return Some(InputEvent::MousePress(MousePress(self.cursor_position)));
        }
        None
    }

    fn handle_mouse_input(
        &mut self,
        _device_id: DeviceId,
        state: ElementState,
        button: MouseButton,
    ) -> Option<InputEvent> {
        self.mouse_left_state = state;

        if state == ElementState::Pressed && button == MouseButton::Left {
            return Some(InputEvent::MouseClick(MouseClick(self.cursor_position)));
        }
        None
    }

    fn handle_keyboard_input(&mut self, _device_id: DeviceId, input: KeyEvent) -> Option<Input> {
        log::debug!("<cyan><b>Keyboard input:</>\n<bright_black>{input:?}</>");

        if input.state != ElementState::Pressed {
            return None;
        }

        let event = 'map_event: {
            let event = match input.logical_key.as_ref() {
                Key::Character(":") => InputEvent::EnterCommand,
                Key::Enter => InputEvent::ExecuteCommand,
                Key::Escape => InputEvent::ExitMode,

                Key::Character("p") => InputEvent::ChangeMode(Mode::Point),
                Key::Character("s") => InputEvent::ChangeMode(Mode::PointSelect),
                Key::Character("a") => InputEvent::AddCurve(Add),
                Key::Character("d") => InputEvent::Delete(Delete),

                Key::Character("j") => InputEvent::ChangeIndex(ChangeIndex(Change::Decrease)),
                Key::Character("k") => InputEvent::ChangeIndex(ChangeIndex(Change::Increase)),

                Key::Character("i") => InputEvent::ChangeWeight(ChangeWeight(Change::Increase)),
                Key::Character("o") => InputEvent::ChangeWeight(ChangeWeight(Change::Decrease)),

                Key::Character("h") => InputEvent::ToggleConvexHull(ToggleConvexHull),

                Key::ArrowUp => InputEvent::MovePoint(MovePoint(Direction::Up)),
                Key::ArrowDown => InputEvent::MovePoint(MovePoint(Direction::Down)),
                Key::ArrowLeft => InputEvent::MovePoint(MovePoint(Direction::Left)),
                Key::ArrowRight => InputEvent::MovePoint(MovePoint(Direction::Right)),

                _ => break 'map_event None,
            };
            Some(event)
        };
        Some(Input::new(event, input.text))
    }
}

impl Default for WindowEventHandler {
    fn default() -> Self {
        Self::new()
    }
}
