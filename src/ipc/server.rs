use std::net::Shutdown;
use std::path::Path;
use std::{fs, slice};

use anyhow::Result;
use async_net::unix::UnixListener;
use futures_lite::{AsyncReadExt, AsyncWriteExt, StreamExt};

use crate::command::message::{Message, MessageType};
use crate::command::program_view::ProgramView;
use crate::ipc::{Status, STATUS_EMPTY, STATUS_ERROR, STATUS_INFO};
use crate::ui::handler::message::{HandlerMessage, HandlerSender};
use crate::ui::shared::SharedState;
use crate::{command, executor};

type IpcReply = (Status, Option<String>);

pub struct IpcServer {
    state: SharedState,
    sender: HandlerSender,
}

impl IpcServer {
    pub fn run(path: impl AsRef<Path>, state: SharedState, sender: HandlerSender) -> Result<()> {
        let path = path.as_ref();
        if path.exists() {
            fs::remove_file(path)?;
        }

        let server = Self::new(state, sender);
        let listener = UnixListener::bind(path)?;
        let listen_future = server.listen(listener);

        let task = executor::spawn(listen_future);
        task.detach();
        Ok(())
    }

    fn new(state: SharedState, sender: HandlerSender) -> Self {
        Self { state, sender }
    }

    async fn listen(mut self, listener: UnixListener) -> Result<()> {
        while let Some(mut stream) = listener.incoming().next().await.transpose()? {
            let mut message = String::new();
            stream.read_to_string(&mut message).await?;
            stream.shutdown(Shutdown::Read)?;

            let (status, reply) = self.handle_message(&message).await?;

            stream.write_all(slice::from_ref(&status)).await?;
            if let Some(reply) = reply {
                stream.write_all(reply.as_bytes()).await?;
            }
            stream.flush().await?;
            stream.shutdown(Shutdown::Write)?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: &str) -> Result<IpcReply> {
        log::debug!("<cyan>IPC command input:</> '{}'", message);

        let sender = HandlerSender::clone(&self.sender);
        let result = {
            let (mut frame, mut tasks) = self.state.lock().await;
            let view = ProgramView::new(sender, &mut frame, &mut tasks);
            command::execute(message, view).transpose()
        };

        // TODO: consider if we should always redraw
        self.sender.send_event(HandlerMessage::Redraw)?;

        let Some(message) = result else {
            return Ok((STATUS_EMPTY, None));
        };
        let message =
            message.unwrap_or_else(|error| Message::new(error.to_string(), MessageType::Error));
        let status = match message.message_type() {
            MessageType::Info => STATUS_INFO,
            MessageType::Error => STATUS_ERROR,
        };
        let message = message.into_text();
        Ok((status, Some(message)))
    }
}
