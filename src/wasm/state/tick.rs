use crate::wasm::wit::control::TickInfo;

#[derive(Debug, Default)]
pub struct TickState {
    number: u64,
}

impl TickState {
    #[must_use]
    pub fn new(number: u64) -> Self {
        Self { number }
    }

    #[must_use]
    pub fn as_tick_info(&self) -> TickInfo {
        TickInfo { number: self.number }
    }

    #[must_use]
    pub fn number(&self) -> u64 {
        self.number
    }
}
