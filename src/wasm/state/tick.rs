use crate::wasm::wit::control::TickInfo;

#[derive(Debug, Default)]
pub struct TickState {
    number: u64,
}

impl TickState {
    pub fn new(number: u64) -> Self {
        Self { number }
    }

    pub fn as_tick_info(&self) -> TickInfo {
        TickInfo { number: self.number }
    }

    pub fn number(&self) -> u64 {
        self.number
    }
}
