use std::path::Path;
use anyhow::{anyhow, Result};
use async_channel::{Receiver, Sender};
use wasmtime::component::{Component, Linker};
use wasmtime::{component, Config, Engine, Store};

use askew::Host;

use crate::wasm::askew::CurveId;
use crate::wasm::request::{Request, Response};
use crate::ui::runner::window_request::{EventLoopProxy, WindowRequest};

pub mod request;

component::bindgen!({
    path: "wit/askew.wit",
    async: true
});

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

    pub async fn run(
        &self,
        path: impl AsRef<Path>,
        proxy: EventLoopProxy,
        sender: Sender<Request>,
        receiver: Receiver<Response>,
    ) -> Result<u32> {
        let mut linker = Linker::new(&self.engine);
        Askew::add_to_linker(&mut linker, |state: &mut State| state)?;

        let state = State::new(proxy, sender, receiver);
        let mut store = Store::new(&self.engine, state);

        let component = Component::from_file(&self.engine, path)?;
        let (bindings, _) = Askew::instantiate_async(&mut store, &component, &linker).await?;

        bindings
            .call_run(&mut store)
            .await?
            .map_err(|_error| anyhow!("script finished with error"))?;
        Ok(42)
    }
}

struct State {
    proxy: EventLoopProxy,
    _sender: Sender<Request>,
    receiver: Receiver<Response>,
}

impl State {
    pub fn new(
        proxy: EventLoopProxy,
        sender: Sender<Request>,
        receiver: Receiver<Response>,
    ) -> Self {
        Self {
            proxy,
            _sender: sender,
            receiver,
        }
    }
}

#[async_trait::async_trait]
impl Host for State {
    async fn rotate_curve(&mut self, id: CurveId, angle_radians: f32) -> Result<()> {
        self.proxy
            .send_event(WindowRequest::WasmRequest(Request::RotateCurve {
                id: id as usize,
                angle: angle_radians,
            }))?;
        let Response::Empty = self.receiver.recv().await?;
        Ok(())
    }

    async fn sleep(&mut self) -> Result<()> {
        self.proxy
            .send_event(WindowRequest::WasmRequest(Request::Sleep { seconds: 3 }))?;
        let Response::Empty = self.receiver.recv().await?;
        Ok(())
    }
}
