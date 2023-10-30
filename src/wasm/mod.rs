use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2;
use wasmtime_wasi::preview2::{Table, WasiCtxBuilder};

use state::State;

use crate::ui::handler::message::HandlerSender;
use crate::ui::task::lock::TaskLock;
use crate::ui::task::{TaskId, TaskResult};
use crate::wasm::wit::{Askew, RunArgument};

pub mod request;
pub mod state;
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
        preview2::command::add_to_linker(&mut linker)?;
        Askew::add_to_linker(&mut linker, |state| state)?;
        let linker = Arc::new(linker);

        Ok(Self { engine, linker })
    }

    pub fn create_task(
        &self,
        path: impl AsRef<Path>,
        task_id: TaskId,
        runner: HandlerSender,
        lock: TaskLock,
    ) -> Result<WasmTask> {
        let wasi_table = Table::new();
        let wasi_context = WasiCtxBuilder::new().build();

        let state = State::new(task_id, runner, lock, wasi_table, wasi_context);
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
