use crate::ui::command_state::CommandState;
use crate::ui::frame::Frame;

pub struct WindowView<'a> {
    pub frame: &'a Frame,
    pub command: &'a CommandState,
}

impl<'a> WindowView<'a> {
    #[must_use]
    pub fn new(frame: &'a Frame, command: &'a CommandState) -> Self {
        Self { frame, command }
    }
}
