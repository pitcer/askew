use crate::ui::frame::Frame;
use crate::ui::handler::message::HandlerSender;
use crate::ui::task::Tasks;

pub struct ProgramView<'a> {
    pub handler_sender: HandlerSender,
    pub frame: &'a mut Frame,
    pub tasks: &'a mut Tasks,
}

impl<'a> ProgramView<'a> {
    pub fn new(handler_sender: HandlerSender, frame: &'a mut Frame, tasks: &'a mut Tasks) -> Self {
        Self { handler_sender, frame, tasks }
    }
}
