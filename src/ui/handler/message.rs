use anyhow::Result;
use winit::event_loop::EventLoopProxy;

use crate::ui::task::TaskId;
use crate::wasm::state::YieldResponse;
use crate::wasm::wit::RunResult;

pub type HandlerSender = EventLoopProxy<HandlerMessage>;

#[derive(Debug)]
pub enum HandlerMessage {
    TaskFinished(TaskId, Result<RunResult>),
    TaskYield(YieldResponse),
    Redraw,
    Exit,
}
