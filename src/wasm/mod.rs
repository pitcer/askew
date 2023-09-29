use anyhow::{anyhow, Result};
use std::thread;
use std::time::Duration;
use wasmtime::component::{Component, Linker};
use wasmtime::{component, Config, Engine, Store};

use askew::Host;

use crate::wasm::askew::CurveId;
use crate::wasm::request::Request;
use crate::window_request::{EventLoopProxy, WindowRequest};

pub mod request;

component::bindgen!({
    path: "wit/askew.wit",
    async: true
});

pub struct Runtime;

impl Runtime {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self, path: &str, proxy: &EventLoopProxy) -> Result<()> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        let mut linker = Linker::new(&engine);
        Askew::add_to_linker(&mut linker, |state: &mut State<'_>| state)?;

        let mut store = Store::new(&engine, State { proxy });

        let component = Component::from_file(&engine, path)?;
        let (bindings, _) = Askew::instantiate_async(&mut store, &component, &linker).await?;

        bindings
            .call_run(&mut store)
            .await?
            .map_err(|_error| anyhow!("script finished with error"))?;
        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

struct State<'a> {
    proxy: &'a EventLoopProxy,
}

#[async_trait::async_trait]
impl<'a> Host for State<'a> {
    async fn rotate_curve(&mut self, id: CurveId, angle_radians: f32) -> Result<()> {
        self.proxy
            .send_event(WindowRequest::WasmRequest(Request::RotateCurve {
                id: id as usize,
                angle: angle_radians,
            }))?;
        Ok(())
    }

    async fn sleep(&mut self) -> Result<()> {
        thread::sleep(Duration::new(3, 0));
        Ok(())
    }
}
