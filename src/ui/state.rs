use crate::ui::frame::Frame;
use crate::ui::mode::ModeState;

#[derive(Debug)]
pub struct ProgramState<'a> {
    pub mode: &'a mut ModeState,
    pub frame: &'a mut Frame,
}

impl<'a> ProgramState<'a> {
    pub fn new(mode: &'a mut ModeState, frame: &'a mut Frame) -> Self {
        Self { mode, frame }
    }
}
