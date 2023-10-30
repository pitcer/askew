use crate::ui::frame::Frame;
use crate::ui::task::Tasks;
use std::sync::Arc;

type Mutex<T> = async_lock::Mutex<T>;

pub type SharedState = Arc<Mutex<State>>;

#[derive(Debug)]
pub struct State {
    pub frame: Frame,
    pub tasks: Tasks,
}

impl State {
    #[must_use]
    pub fn new(frame: Frame, tasks: Tasks) -> SharedState {
        let state = Self { frame, tasks };
        Arc::new(Mutex::new(state))
    }
}
