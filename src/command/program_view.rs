use crate::ui::frame::Frame;
use crate::ui::mode::ModeState;
use crate::ui::runner::task::Tasks;
use winit::event_loop::ControlFlow;

pub struct ProgramView<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub mode: &'a mut ModeState,
    pub frame: &'a mut Frame,
    pub tasks: &'a mut Tasks,
}

impl<'a> ProgramView<'a> {
    pub fn new(
        control_flow: &'a mut ControlFlow,
        mode: &'a mut ModeState,
        frame: &'a mut Frame,
        tasks: &'a mut Tasks,
    ) -> Self {
        Self { control_flow, mode, frame, tasks }
    }
}
