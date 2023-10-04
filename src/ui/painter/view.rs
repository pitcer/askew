use crate::command::CommandState;
use crate::ui::frame::Frame;
use crate::ui::mode::ModeState;

pub struct WindowView<'a> {
    pub frame: &'a Frame,
    pub command: &'a CommandState,
    pub mode: &'a ModeState,
}

impl<'a> WindowView<'a> {
    #[must_use]
    pub fn new(frame: &'a Frame, command: &'a CommandState, mode: &'a ModeState) -> Self {
        Self { frame, command, mode }
    }
}
