use async_task::Runnable;
use winit::event_loop;

use crate::ipc::server::IpcMessage;
use crate::wasm::request::Request;

pub type EventLoopProxy = event_loop::EventLoopProxy<WindowRequest>;

#[derive(Debug)]
pub enum WindowRequest {
    NoReplyCommand(String),
    IpcMessage(IpcMessage),
    WasmRequest(Request),
    WasmRun { path: String },
    Progress(Runnable),
}
