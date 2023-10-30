use std::sync::Arc;

use crate::ui::frame::Frame;
use crate::ui::task::Tasks;

/// The purpose of this container is to ensure lock order on the stored state. Only `frame` can be
/// cloned and used separately, because it's locked before other fields.
#[derive(Debug, Clone)]
pub struct SharedState {
    frame: SharedFrame,
    tasks: SharedTasks,
}

pub type SharedFrame = Shared<Frame>;
pub type SharedTasks = Shared<Tasks>;

#[derive(Debug)]
pub struct Shared<T>(Arc<Mutex<T>>);

type Mutex<T> = async_lock::Mutex<T>;
type MutexGuard<T> = async_lock::MutexGuardArc<T>;

impl SharedState {
    #[must_use]
    pub fn new(frame: SharedFrame, tasks: SharedTasks) -> Self {
        Self { frame, tasks }
    }

    pub async fn lock(&self) -> (MutexGuard<Frame>, MutexGuard<Tasks>) {
        let frame = self.frame.lock().await;
        let tasks = self.tasks.lock().await;
        (frame, tasks)
    }

    #[must_use]
    pub fn lock_blocking(&self) -> (MutexGuard<Frame>, MutexGuard<Tasks>) {
        let frame = self.frame.lock_blocking();
        let tasks = self.tasks.lock_blocking();
        (frame, tasks)
    }

    #[must_use]
    pub fn frame(&self) -> SharedFrame {
        SharedFrame::clone(&self.frame)
    }
}

impl<T> Shared<T> {
    pub fn new(shared: T) -> Self {
        let shared = Arc::new(Mutex::new(shared));
        Self(shared)
    }

    pub async fn lock(&self) -> MutexGuard<T> {
        self.0.lock_arc().await
    }

    #[must_use]
    pub fn lock_blocking(&self) -> MutexGuard<T> {
        self.0.lock_arc_blocking()
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        let clone = Arc::clone(&self.0);
        Self(clone)
    }
}
