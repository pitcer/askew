use std::io::Write;
use std::net::Shutdown;
use std::os::unix::net::{SocketAddr, UnixListener, UnixStream};
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{fs, io, slice, thread};

use anyhow::Result;
use winit::event_loop::EventLoopProxy;

use crate::ipc::{Status, STATUS_EMPTY, STATUS_ERROR, STATUS_INFO};
use crate::ui::command::interpreter::CommandInterpreter;
use crate::ui::command::message::{Message, MessageType};
use crate::ui::command::parser::CommandParser;
use crate::ui::state::ProgramState;

pub type IpcReply = Option<(Status, Option<String>)>;

pub struct IpcServer {
    proxy: EventLoopProxy<IpcMessage>,
    receiver: Receiver<IpcReply>,
}

impl IpcServer {
    fn new(proxy: EventLoopProxy<IpcMessage>, receiver: Receiver<IpcReply>) -> Self {
        Self { proxy, receiver }
    }

    pub fn run(
        path: impl AsRef<Path>,
        proxy: EventLoopProxy<IpcMessage>,
    ) -> Result<IpcServerHandle> {
        let path = path.as_ref();
        if path.exists() {
            fs::remove_file(path)?;
        }

        let (sender, receiver) = mpsc::channel();
        let server = Self::new(proxy, receiver);

        let listener = UnixListener::bind(path)?;
        let address = listener.local_addr()?;

        let handle = thread::spawn(|| server.listen(listener));
        let handle = IpcServerHandle::new(address, handle, sender);
        Ok(handle)
    }

    fn listen(self, listener: UnixListener) -> Result<()> {
        loop {
            let (mut stream, _) = listener.accept()?;

            let message = io::read_to_string(&stream)?;
            stream.shutdown(Shutdown::Read)?;

            let message = IpcMessage::new(message);
            self.proxy.send_event(message)?;
            let Some((status, reply)) = self.receiver.recv()? else { break; };

            stream.write_all(slice::from_ref(&status))?;
            if let Some(reply) = reply {
                stream.write_all(reply.as_bytes())?;
            }
            stream.flush()?;
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
    pub fn handle(mut self, state: ProgramState<'_>) -> IpcReply {
        let result = self.interpret(state).transpose();
        let Some(message) = result else {
            return Some((STATUS_EMPTY, None));
        };
        let message =
            message.unwrap_or_else(|error| Message::new(error.to_string(), MessageType::Error));
        let status = match message.message_type() {
            MessageType::Info => STATUS_INFO,
            MessageType::Error => STATUS_ERROR,
        };
        let message = message.into_text();
        Some((status, Some(message)))
    }

    fn interpret(&mut self, state: ProgramState<'_>) -> Result<Option<Message>> {
        let mut parser = CommandParser::new(&self.message);
        let result = parser.parse()?;
        let mut interpreter = CommandInterpreter::new(state);
        let message = interpreter.interpret(result)?;
        Ok(message)
    }
}

pub struct IpcServerHandle {
    address: SocketAddr,
    join_handle: JoinHandle<Result<()>>,
    sender: Sender<IpcReply>,
}

impl IpcServerHandle {
    fn new(
        address: SocketAddr,
        join_handle: JoinHandle<Result<()>>,
        sender: Sender<IpcReply>,
    ) -> Self {
        Self {
            address,
            join_handle,
            sender,
        }
    }

    pub fn send(&self, reply: IpcReply) -> Result<()> {
        self.sender.send(reply)?;
        Ok(())
    }

    pub fn close(self) -> Result<()> {
        // Connect and shutdown to pass blocking accept and read calls in listen function.
        let stream = UnixStream::connect_addr(&self.address)?;
        stream.shutdown(Shutdown::Both)?;

        // Causes break in listen loop.
        self.sender.send(None)?;

        self.join_handle
            .join()
            .expect("handle should not fail on join")?;
        Ok(())
    }
}
