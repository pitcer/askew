use async_executor::Executor;
use async_task::Task;
use std::future::Future;
use std::sync::OnceLock;

static GLOBAL_EXECUTOR: OnceLock<AsyncExecutor<'static>> = OnceLock::new();

#[derive(Debug, Default)]
pub struct AsyncExecutor<'a> {
    executor: Executor<'a>,
}

impl<'a> AsyncExecutor<'a> {
    #[must_use]
    pub fn new() -> Self {
        let executor = Executor::new();
        Self { executor }
    }

    pub fn block_on_run<T>(&self, future: impl Future<Output = T>) -> T {
        let run = self.executor.run(future);
        async_io::block_on(run)
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'a) -> Task<T>
    where
        T: Send + 'a,
    {
        self.executor.spawn(future)
    }
}

pub fn spawn<T>(future: impl Future<Output = T> + Send + 'static) -> Task<T>
where
    T: Send + 'static,
{
    let executor = GLOBAL_EXECUTOR.get_or_init(AsyncExecutor::new);
    executor.spawn(future)
}

pub fn block_on_run<T>(future: impl Future<Output = T>) -> T {
    let executor = GLOBAL_EXECUTOR.get_or_init(AsyncExecutor::new);
    executor.block_on_run(future)
}
