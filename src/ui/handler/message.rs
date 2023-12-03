use async_channel::Sender;
use winit::event_loop::EventLoopProxy;

use crate::ui::task::TaskId;
use crate::wasm::wit::RunResult;

pub type HandlerSender = EventLoopProxy<HandlerMessage>;

#[derive(Debug)]
pub enum HandlerMessage {
    TaskFinished(TaskId, RunResult),
    TaskYield(YieldResponse),
    Redraw,
    Exit,
}

#[derive(Debug)]
pub struct YieldResponse(Sender<()>);

impl YieldResponse {
    #[must_use]
    pub fn new(sender: Sender<()>) -> Self {
        Self(sender)
    }

    pub fn send(&self) {
        self.0.try_send(()).expect("Cannot send yield response");
    }
}
