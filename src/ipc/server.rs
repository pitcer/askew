use std::net::Shutdown;
use std::path::Path;
use std::{fs, slice};

use anyhow::Result;
use async_channel::{Receiver, Sender};
use async_net::unix::UnixListener;
use async_task::Task;
use futures_lite::{AsyncReadExt, AsyncWriteExt, StreamExt};

use crate::command;
use crate::command::message::{Message, MessageType};
use crate::command::program_view::ProgramView;
use crate::ipc::{Status, STATUS_EMPTY, STATUS_ERROR, STATUS_INFO};
use crate::ui::runner::window_request::{EventLoopRequest, RunnerSender};

pub type IpcReply = (Status, Option<String>);

pub type ServerTask = Task<Result<()>>;

pub struct IpcServer {
    proxy: RunnerSender,
    receiver: Receiver<IpcReply>,
}

impl IpcServer {
    pub fn run(path: impl AsRef<Path>, proxy: RunnerSender) -> Result<IpcServerHandle> {
        let path = path.as_ref();
        if path.exists() {
            fs::remove_file(path)?;
        }

        let (sender, receiver) = async_channel::unbounded();
        let server = Self::new(proxy.clone(), receiver);

        let listener = UnixListener::bind(path)?;

        let future = async move { server.listen(listener).await };
        let schedule = move |runnable| {
            proxy.send_event(EventLoopRequest::ProgressIpcServer(runnable)).unwrap();
        };
        let (runnable, task) = async_task::spawn(future, schedule);
        runnable.schedule();

        let handle = IpcServerHandle::new(task, sender);
        Ok(handle)
    }

    fn new(proxy: RunnerSender, receiver: Receiver<IpcReply>) -> Self {
        Self { proxy, receiver }
    }

    async fn listen(self, listener: UnixListener) -> Result<()> {
        while let Some(mut stream) = listener.incoming().next().await.transpose()? {
            let mut message = String::new();
            stream.read_to_string(&mut message).await?;
            stream.shutdown(Shutdown::Read)?;

            let message = IpcMessage::new(message);
            self.proxy.send_event(EventLoopRequest::IpcMessage(message))?;
            let (status, reply) = self.receiver.recv().await?;

            stream.write_all(slice::from_ref(&status)).await?;
            if let Some(reply) = reply {
                stream.write_all(reply.as_bytes()).await?;
            }
            stream.flush().await?;
            stream.shutdown(Shutdown::Write)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct IpcMessage {
    message: String,
}

impl IpcMessage {
    fn new(message: String) -> IpcMessage {
        Self { message }
    }

    #[must_use]
    pub fn handle(self, state: ProgramView<'_>) -> IpcReply {
        let result = command::execute(&self.message, state).transpose();
        let Some(message) = result else {
            return (STATUS_EMPTY, None);
        };
        let message =
            message.unwrap_or_else(|error| Message::new(error.to_string(), MessageType::Error));
        let status = match message.message_type() {
            MessageType::Info => STATUS_INFO,
            MessageType::Error => STATUS_ERROR,
        };
        let message = message.into_text();
        (status, Some(message))
    }
}

pub struct IpcServerHandle {
    task: ServerTask,
    sender: Sender<IpcReply>,
}

impl IpcServerHandle {
    fn new(task: ServerTask, sender: Sender<IpcReply>) -> Self {
        Self { task, sender }
    }

    pub fn send(&self, reply: IpcReply) -> Result<()> {
        self.sender.send_blocking(reply)?;
        Ok(())
    }

    pub fn close(self) {
        drop(self.task);
    }
}
