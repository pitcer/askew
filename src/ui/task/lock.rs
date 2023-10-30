use std::sync::Arc;

use anyhow::Result;
use async_lock::{Mutex, MutexGuardArc};

#[derive(Debug, Clone)]
pub struct TaskLock {
    lock: Arc<Mutex<LockData>>,
}

#[derive(Debug)]
pub struct LockToken(MutexGuardArc<LockData>);

type LockData = ();

impl TaskLock {
    pub fn new() -> Result<Self> {
        let lock = Arc::new(Mutex::new(()));
        Ok(Self { lock })
    }

    pub async fn acquire(&self) -> LockToken {
        let guard = self.lock.lock_arc().await;
        LockToken(guard)
    }

    pub fn release(&self, token: LockToken) {
        drop(token);
    }
}
