use winit::event_loop::EventLoopWindowTarget;

use crate::ui::frame::Frame;
use crate::ui::handler::message::HandlerMessage;
use crate::ui::task::Tasks;

pub struct ProgramView<'a> {
    pub target: &'a EventLoopWindowTarget<HandlerMessage>,
    pub frame: &'a mut Frame,
    pub tasks: &'a mut Tasks,
}

impl<'a> ProgramView<'a> {
    pub fn new(
        target: &'a EventLoopWindowTarget<HandlerMessage>,
        frame: &'a mut Frame,
        tasks: &'a mut Tasks,
    ) -> Self {
        Self { target, frame, tasks }
    }
}
