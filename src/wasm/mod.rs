use std::path::Path;

use anyhow::{anyhow, Result};
use async_channel::Sender;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};

use crate::ui::runner::window_request::{EventLoopRequest, EventLoopSender};
use crate::wasm::request::{Request, Response};
use crate::wasm::wit::curve::CurveId;
use crate::wasm::wit::{control, curve, Askew};

pub mod request;
pub mod wit;

pub struct WasmRuntime {
    engine: Engine,
}

impl WasmRuntime {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        Ok(Self { engine })
    }

    pub async fn run(&self, path: impl AsRef<Path>, sender: EventLoopSender) -> Result<u32> {
        let mut linker = Linker::new(&self.engine);
        Askew::add_to_linker(&mut linker, |state: &mut State| state)?;

        let state = State::new(sender);
        let mut store = Store::new(&self.engine, state);

        let component = Component::from_file(&self.engine, path)?;
        let (bindings, _) = Askew::instantiate_async(&mut store, &component, &linker).await?;

        let result = bindings.call_run(&mut store).await?;
        result.map_err(|_error| anyhow!("script finished with error"))?;
        Ok(42)
    }
}

struct State {
    sender: EventLoopSender,
}

impl State {
    pub fn new(sender: EventLoopSender) -> Self {
        Self { sender }
    }

    pub async fn send_request(&mut self, request: Request) -> Result<Response> {
        let (response_sender, response_receiver) = async_channel::bounded(1);
        let request = RequestHandle::new(request, response_sender);
        let request = EventLoopRequest::TaskRequest(request);
        self.sender.send_event(request)?;
        let response = response_receiver.recv().await?;
        Ok(response)
    }
}

#[derive(Debug)]
pub struct RequestHandle {
    pub request: Request,
    pub response_sender: Sender<Response>,
}

impl RequestHandle {
    #[must_use]
    pub fn new(request: Request, response_sender: Sender<Response>) -> Self {
        Self { request, response_sender }
    }

    pub fn respond(&self, response: Response) -> Result<()> {
        self.response_sender.send_blocking(response)?;
        Ok(())
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
        let Response::Sleep = self.send_request(request).await? else {
            return Err(anyhow!("Invalid response"));
        };
        Ok(())
    }
}
