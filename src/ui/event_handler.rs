use anyhow::Result;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    DeviceId, ElementState, Event as WinitEvent, KeyboardInput, ModifiersState, MouseButton,
    VirtualKeyCode, WindowEvent,
};
use winit::event_loop::ControlFlow;

use crate::canvas::math::vector::Vector;
use crate::canvas::mode::Mode;
use crate::command::{Command, SaveFormat};
use crate::event::{CanvasEvent, CurveEvent, Event, FrameEvent};
use crate::ui::frame::Frame;

pub struct EventHandler {
    frame: Frame,
    cursor_position: PhysicalPosition<f64>,
    modifiers: ModifiersState,
    save_format: Option<SaveFormat>,
}

impl EventHandler {
    pub fn new(frame: Frame, command: &Command) -> Self {
        let cursor_position = PhysicalPosition::new(0.0, 0.0);
        let modifiers = ModifiersState::empty();
        Self {
            frame,
            cursor_position,
            modifiers,
            save_format: command.save_format,
        }
    }

    pub fn run(&mut self, event: WinitEvent<()>, control_flow: &mut ControlFlow) -> Result<()> {
        control_flow.set_wait();

        match event {
            WinitEvent::RedrawRequested(window_id) if self.frame.has_id(window_id) => {
                self.frame.draw()?;
            }
            WinitEvent::WindowEvent { event, window_id } if self.frame.has_id(window_id) => {
                let event = self.handle_window_event(event, control_flow)?;
                self.frame.handle_event(event)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_window_event(
        &mut self,
        event: WindowEvent,
        control_flow: &mut ControlFlow,
    ) -> Result<Option<Event>> {
        match event {
            WindowEvent::Resized(size) => {
                self.handle_resized(size)?;
            }
            WindowEvent::CloseRequested => {
                if let Some(format) = self.save_format {
                    self.frame.save(format)?;
                }
                control_flow.set_exit()
            }
            WindowEvent::ReceivedCharacter(character) => {
                return Ok(Some(Event::Frame(FrameEvent::ReceiveCharacter(character))));
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
    ) -> Option<Event> {
        if state == ElementState::Pressed && button == MouseButton::Left {
            return Some(Event::Canvas(CanvasEvent::Curve(CurveEvent::AddPoint(
                self.cursor_position,
            ))));
        }
        None
    }

    fn handle_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        input: KeyboardInput,
    ) -> Option<Event> {
        if input.state != ElementState::Pressed {
            return None;
        }
        match input.virtual_keycode {
            Some(VirtualKeyCode::Colon) => Some(Event::Frame(FrameEvent::EnterCommand)),
            Some(VirtualKeyCode::Return) => Some(Event::Frame(FrameEvent::ExecuteCommand)),
            Some(VirtualKeyCode::Escape) => Some(Event::Frame(FrameEvent::ExitMode)),

            Some(VirtualKeyCode::C) => Some(Event::Canvas(CanvasEvent::ChangeMode(Mode::Curve))),
            Some(VirtualKeyCode::A) => Some(Event::Canvas(CanvasEvent::Add)),
            Some(VirtualKeyCode::D) => Some(Event::Canvas(CanvasEvent::Delete)),

            Some(VirtualKeyCode::J) => Some(Event::Canvas(CanvasEvent::ChangeIndex(-1))),
            Some(VirtualKeyCode::K) => Some(Event::Canvas(CanvasEvent::ChangeIndex(1))),

            Some(VirtualKeyCode::I) => Some(Event::Canvas(CanvasEvent::Curve(
                CurveEvent::ChangeWeight(1.5),
            ))),
            Some(VirtualKeyCode::O) => Some(Event::Canvas(CanvasEvent::Curve(
                CurveEvent::ChangeWeight(-1.5),
            ))),

            Some(VirtualKeyCode::H) => Some(Event::Canvas(CanvasEvent::Curve(
                CurveEvent::ToggleConvexHull,
            ))),

            Some(VirtualKeyCode::Up) => Some(Event::Canvas(CanvasEvent::Curve(
                CurveEvent::MoveCurrentPoint(Vector::new(0.0, -4.0)),
            ))),
            Some(VirtualKeyCode::Down) => Some(Event::Canvas(CanvasEvent::Curve(
                CurveEvent::MoveCurrentPoint(Vector::new(0.0, 4.0)),
            ))),
            Some(VirtualKeyCode::Left) => Some(Event::Canvas(CanvasEvent::Curve(
                CurveEvent::MoveCurrentPoint(Vector::new(-4.0, 0.0)),
            ))),
            Some(VirtualKeyCode::Right) => Some(Event::Canvas(CanvasEvent::Curve(
                CurveEvent::MoveCurrentPoint(Vector::new(4.0, 0.0)),
            ))),
            _ => None,
        }
    }
}
