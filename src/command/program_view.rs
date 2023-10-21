use winit::event_loop::EventLoopWindowTarget;

use crate::ui::frame::Frame;
use crate::ui::runner::request::RunnerRequest;
use crate::ui::runner::task::Tasks;

pub struct ProgramView<'a> {
    pub target: &'a EventLoopWindowTarget<RunnerRequest>,
    pub frame: &'a mut Frame,
    pub tasks: &'a mut Tasks,
}

impl<'a> ProgramView<'a> {
    pub fn new(
        target: &'a EventLoopWindowTarget<RunnerRequest>,
        frame: &'a mut Frame,
        tasks: &'a mut Tasks,
    ) -> Self {
        Self { target, frame, tasks }
    }
}
