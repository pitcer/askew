use anyhow::{anyhow, Result};
use async_channel::{Sender, TrySendError};
use async_io::Timer;
use std::time::Duration;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView};

use crate::ui::handler::message::{HandlerMessage, HandlerSender};
use crate::ui::task::lock::{LockToken, TaskLock};
use crate::ui::task::TaskId;
use crate::wasm::request::{Request, Response};
use crate::wasm::wit::curve::CurveId;
use crate::wasm::wit::{control, curve};

pub struct State {
    task_id: TaskId,
    runner: HandlerSender,
    lock: TaskLock,
    lock_token: Option<LockToken>,

    wasi_table: Table,
    wasi_context: WasiCtx,
}

impl State {
    #[must_use]
    pub fn new(
        task_id: TaskId,
        runner: HandlerSender,
        lock: TaskLock,
        wasi_table: Table,
        wasi_context: WasiCtx,
    ) -> Self {
        Self { task_id, runner, lock, lock_token: None, wasi_table, wasi_context }
    }

    pub async fn send_request(&mut self, request: Request) -> Result<Response> {
        let response = if self.lock_token.is_some() {
            self.send_request_lockless(request).await?
        } else {
            let token = self.lock.acquire().await;
            let response = self.send_request_lockless(request).await?;
            self.lock.release(token);
            response
        };
        Ok(response)
    }

    pub async fn send_request_lockless(&mut self, request: Request) -> Result<Response> {
        let (response_sender, response_receiver) = async_channel::bounded(1);
        let responder = Responder::new(self.task_id, response_sender);
        let request = RequestHandle::new(request, responder);
        let request = HandlerMessage::TaskRequest(request);
        self.runner.send_event(request)?;

        let response = response_receiver.recv().await?;
        Ok(response)
    }
}

#[derive(Debug)]
pub struct Responder {
    task_id: TaskId,
    response_sender: Sender<Response>,
}

impl Responder {
    #[must_use]
    pub fn new(task_id: TaskId, response_sender: Sender<Response>) -> Self {
        Self { task_id, response_sender }
    }

    pub fn respond(&self, response: Response) {
        let result = self.response_sender.try_send(response);
        match result {
            Err(TrySendError::Full(response)) => {
                log::warn!(
                    "Cannot send response `{response:?}` to task {}, because it was already sent.",
                    self.task_id
                );
            }
            Err(TrySendError::Closed(response)) => {
                log::warn!(
                    "Cannot send response `{response:?}` to task {}, because receiver was closed.",
                    self.task_id
                );
            }
            Ok(()) => (),
        }
    }
}

#[derive(Debug)]
pub struct RequestHandle {
    pub request: Request,
    pub responder: Responder,
}

impl RequestHandle {
    #[must_use]
    pub fn new(request: Request, responder: Responder) -> Self {
        Self { request, responder }
    }
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.wasi_table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.wasi_table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.wasi_context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_context
    }
}

#[async_trait::async_trait]
impl curve::Host for State {
    async fn get_position(&mut self, id: CurveId) -> Result<(f32, f32)> {
        let request = Request::GetPosition { id };
        let Response::GetPosition { horizontal, vertical } = self.send_request(request).await?
        else {
            return Err(anyhow!("Invalid response"));
        };
        Ok((horizontal, vertical))
    }

    async fn move_by(&mut self, id: CurveId, horizontal: f32, vertical: f32) -> Result<()> {
        let request = Request::MoveCurve { id, horizontal, vertical };
        let Response::Empty = self.send_request(request).await? else {
            return Err(anyhow!("Invalid response"));
        };
        Ok(())
    }

    async fn rotate_by(&mut self, id: CurveId, angle_radians: f32) -> Result<()> {
        let request = Request::RotateCurve { id, angle_radians };
        let Response::Empty = self.send_request(request).await? else {
            return Err(anyhow!("Invalid response"));
        };
        Ok(())
    }
}

#[async_trait::async_trait]
impl control::Host for State {
    async fn sleep(&mut self, seconds: u64, nanoseconds: u32) -> Result<()> {
        let duration = Duration::new(seconds, nanoseconds);
        let timer = Timer::after(duration);
        timer.await;
        Ok(())
    }

    async fn lock(&mut self) -> Result<()> {
        let None = self.lock_token else {
            return Err(anyhow!("Tried to lock without unlocking first"));
        };
        let token = self.lock.acquire().await;
        self.lock_token = Some(token);
        Ok(())
    }

    async fn unlock(&mut self) -> Result<()> {
        let Some(token) = self.lock_token.take() else {
            return Err(anyhow!("Tried to unlock without locking first"));
        };
        self.lock.release(token);
        Ok(())
    }
}
