use crate::ui::frame::Frame;
use crate::ui::mode::ModeState;
use crate::ui::runner::task::Tasks;

pub struct ProgramView<'a> {
    pub mode: &'a mut ModeState,
    pub frame: &'a mut Frame,
    pub tasks: &'a mut Tasks,
}

impl<'a> ProgramView<'a> {
    pub fn new(mode: &'a mut ModeState, frame: &'a mut Frame, tasks: &'a mut Tasks) -> Self {
        Self { mode, frame, tasks }
    }
}
