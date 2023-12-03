use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView};

use crate::ui::handler::message::HandlerSender;
use crate::ui::shared::SharedFrame;
use crate::ui::task::lock::{LockToken, TaskLock};
use crate::wasm::state::tick::TickState;

pub mod control;
pub mod shape;
pub mod tick;

pub struct State {
    sender: HandlerSender,
    frame: SharedFrame,
    lock: TaskLock,
    lock_token: Option<LockToken>,

    wasi_table: Table,
    wasi_context: WasiCtx,

    tick_state: TickState,
}

impl State {
    #[must_use]
    pub fn new(
        sender: HandlerSender,
        frame: SharedFrame,
        lock: TaskLock,
        wasi_table: Table,
        wasi_context: WasiCtx,
    ) -> Self {
        let lock_token = None;
        let tick_state = TickState::default();
        Self { sender, frame, lock, lock_token, wasi_table, wasi_context, tick_state }
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
