use winit::event_loop::ControlFlow;

use crate::ui::frame::Frame;
use crate::ui::runner::task::Tasks;

pub struct ProgramView<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub frame: &'a mut Frame,
    pub tasks: &'a mut Tasks,
}

impl<'a> ProgramView<'a> {
    pub fn new(
        control_flow: &'a mut ControlFlow,
        frame: &'a mut Frame,
        tasks: &'a mut Tasks,
    ) -> Self {
        Self { control_flow, frame, tasks }
    }
}
