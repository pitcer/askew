use crate::ui::frame::Frame;
use crate::ui::mode::ModeState;
use crate::ui::runner::window_request::EventLoopProxy;

#[derive(Debug)]
pub struct ProgramState<'a> {
    pub mode: &'a mut ModeState,
    pub frame: &'a mut Frame,
    pub proxy: &'a EventLoopProxy,
}

impl<'a> ProgramState<'a> {
    pub fn new(mode: &'a mut ModeState, frame: &'a mut Frame, proxy: &'a EventLoopProxy) -> Self {
        Self { mode, frame, proxy }
    }
}
