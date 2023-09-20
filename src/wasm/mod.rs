use anyhow::{anyhow, Result};
use wasmtime::component::{Component, Linker};
use wasmtime::{component, Config, Engine, Store};

use askew::Host;

use crate::wasm::askew::CurveId;
use crate::wasm::request::Request;
use crate::window_request::{EventLoopProxy, WindowRequest};

pub mod request;

component::bindgen!({
    path: "wit/askew.wit",
});

pub struct Runtime;

impl Runtime {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, path: &str, proxy: &EventLoopProxy) -> Result<()> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, path)?;

        let mut linker = Linker::new(&engine);
        Askew::add_to_linker(&mut linker, |state: &mut State<'_>| state)?;

        let mut store = Store::new(&engine, State { proxy });
        let (bindings, _) = Askew::instantiate(&mut store, &component, &linker)?;

        bindings
            .call_run(&mut store)?
            .map_err(|_error| anyhow!("script finished with error"))?;
        Ok(())
    }
}

struct State<'a> {
    proxy: &'a EventLoopProxy,
}

impl<'a> Host for State<'a> {
    fn rotate_curve(&mut self, id: CurveId, angle_radians: f32) -> Result<()> {
        self.proxy
            .send_event(WindowRequest::WasmRequest(Request::RotateCurve {
                id: id as usize,
                angle: angle_radians,
            }))?;
        Ok(())
    }
}
