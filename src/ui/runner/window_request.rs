use async_task::Runnable;
use winit::event_loop;

use crate::ipc::server::IpcMessage;
use crate::ui::runner::task::TaskHandle;
use crate::wasm::RequestHandle;

pub type EventLoopSender = event_loop::EventLoopProxy<EventLoopRequest>;

#[derive(Debug)]
pub enum EventLoopRequest {
    NoReplyCommand(String),
    IpcMessage(IpcMessage),
    TaskRequest(RequestHandle),
    ProgressTask(TaskHandle),
    ProgressIpcServer(Runnable),
}
