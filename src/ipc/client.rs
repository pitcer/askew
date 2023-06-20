use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::{io, slice};

use crate::ipc::{STATUS_EMPTY, STATUS_ERROR, STATUS_INFO};

pub struct IpcClient {
    stream: UnixStream,
}

impl IpcClient {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let stream = UnixStream::connect(path)?;
        Ok(Self { stream })
    }

    pub fn send(mut self, message: String) -> anyhow::Result<()> {
        self.stream.write_all(message.as_bytes())?;
        self.stream.flush()?;
        self.stream.shutdown(Shutdown::Write)?;

        let mut status = 0u8;
        self.stream.read_exact(slice::from_mut(&mut status))?;

        match status {
            STATUS_EMPTY => {}
            STATUS_INFO => {
                io::copy(&mut self.stream, &mut io::stdout())?;
            }
            STATUS_ERROR => {
                io::copy(&mut self.stream, &mut io::stderr())?;
            }
            _ => {
                eprintln!("invalid status");
            }
        }

        self.stream.shutdown(Shutdown::Read)?;
        Ok(())
    }
}
