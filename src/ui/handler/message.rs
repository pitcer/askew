use anyhow::Result;
use winit::event_loop::EventLoopProxy;

use crate::ui::task::TaskId;
use crate::wasm::state::RequestHandle;
use crate::wasm::wit::RunResult;

pub type HandlerSender = EventLoopProxy<HandlerMessage>;

#[derive(Debug)]
pub enum HandlerMessage {
    TaskRequest(RequestHandle),
    TaskFinished(TaskId, Result<RunResult>),
    Redraw,
    Exit,
}
