use winit::event_loop::EventLoopProxy;

use crate::ipc::server::IpcMessage;
use crate::ui::runner::task::TaskId;
use crate::wasm::state::RequestHandle;

pub type RunnerSender = EventLoopProxy<HandlerMessage>;

#[derive(Debug)]
pub enum HandlerMessage {
    IpcMessage(IpcMessage),
    TaskRequest(RequestHandle),
    TaskFinished(TaskId),
}
