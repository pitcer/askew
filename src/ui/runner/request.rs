use async_task::Runnable;
use winit::event_loop::EventLoopProxy;

use crate::ipc::server::IpcMessage;
use crate::ui::runner::task::TaskHandle;
use crate::wasm::state::RequestHandle;

pub type RunnerSender = EventLoopProxy<RunnerRequest>;

#[derive(Debug)]
pub enum RunnerRequest {
    IpcMessage(IpcMessage),
    TaskRequest(RequestHandle),
    ProgressTask(TaskHandle),
    ProgressIpcServer(Runnable),
}
