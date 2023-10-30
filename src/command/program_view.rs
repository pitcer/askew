use crate::ui::handler::message::HandlerSender;
use crate::ui::state::State;

pub struct ProgramView<'a> {
    pub handler_sender: HandlerSender,
    pub state: &'a mut State,
}

impl<'a> ProgramView<'a> {
    pub fn new(handler_sender: HandlerSender, state: &'a mut State) -> Self {
        Self { handler_sender, state }
    }
}
