use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_channel::{SendError, Sender};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

use crate::ui::runner::request::{RunnerRequest, RunnerSender};
use crate::ui::runner::task::lock::{LockToken, TaskLock};
use crate::ui::runner::task::{TaskId, TaskResult};
use crate::wasm::request::{Request, Response};
use crate::wasm::wit::curve::CurveId;
use crate::wasm::wit::{control, curve, Askew, RunArgument};

pub mod request;
pub mod wit;

pub struct WasmRuntime {
    engine: Engine,
    linker: Arc<Linker<State>>,
}

impl WasmRuntime {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        Askew::add_to_linker(&mut linker, |state| state)?;
        let linker = Arc::new(linker);

        Ok(Self { engine, linker })
    }

    pub fn create_task(
        &self,
        path: impl AsRef<Path>,
        task_id: TaskId,
        runner: RunnerSender,
        lock: TaskLock,
    ) -> Result<WasmTask> {
        let state = State::new(task_id, runner, lock);
        let store = Store::new(&self.engine, state);
        let component = Component::from_file(&self.engine, path)?;
        let linker = Arc::clone(&self.linker);

        let task = WasmTask::new(store, component, linker);
        Ok(task)
    }
}

pub struct WasmTask {
    store: Store<State>,
    component: Component,
    linker: Arc<Linker<State>>,
}

impl WasmTask {
    fn new(store: Store<State>, component: Component, linker: Arc<Linker<State>>) -> Self {
        Self { store, component, linker }
    }

    pub async fn run(mut self, argument: RunArgument) -> TaskResult {
        let (bindings, _instance) =
            Askew::instantiate_async(&mut self.store, &self.component, &self.linker).await?;

        let result = bindings.call_run(&mut self.store, &argument).await?;
        Ok(result)
    }
}

struct State {
    task_id: TaskId,
    runner: RunnerSender,
    lock: TaskLock,
    lock_token: Option<LockToken>,
}

impl State {
    pub fn new(task_id: TaskId, runner: RunnerSender, lock: TaskLock) -> Self {
        Self { task_id, runner, lock, lock_token: None }
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
        let request = RunnerRequest::TaskRequest(request);
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
        let result = self.response_sender.send_blocking(response);
        if let Err(SendError(response)) = result {
            log::warn!(
                "Cannot send response `{response:?}` to task {}, because receiver was closed.",
                self.task_id
            );
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
        let request = Request::Sleep { seconds, nanoseconds };
        let Response::Sleep = self.send_request_lockless(request).await? else {
            return Err(anyhow!("Invalid response"));
        };
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
