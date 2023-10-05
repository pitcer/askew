use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use async_task::Runnable;
use bitvec::vec::BitVec;
use futures_lite::future;
use winit::event_loop::EventLoopProxy;

use crate::ui::runner::window_request::{EventLoopRequest, RunnerSender};
use crate::wasm::WasmRuntime;

pub struct Tasks {
    tasks: HashMap<TaskId, Task>,
    task_id_mask: TaskIdMask,

    runtime: Arc<WasmRuntime>,
    event_loop_sender: RunnerSender,
}

impl Tasks {
    pub fn new(event_loop_sender: RunnerSender) -> Result<Self> {
        let tasks = HashMap::new();
        let runtime = WasmRuntime::new()?;
        let runtime = Arc::new(runtime);
        let task_id_mask = TaskIdMask::new();
        Ok(Self { tasks, task_id_mask, runtime, event_loop_sender })
    }

    pub fn list_tasks(&self) -> impl Iterator<Item = &TaskId> {
        self.tasks.keys()
    }

    pub fn register_task(&mut self, path: impl AsRef<Path> + Send + 'static) -> TaskId {
        let runtime = Arc::clone(&self.runtime);
        let task_id = self.task_id_mask.crate_task_id();
        let proxy = EventLoopProxy::clone(&self.event_loop_sender);
        let future = async move { runtime.run(path, task_id, proxy).await };

        let proxy = self.event_loop_sender.clone();
        let schedule = move |runnable| {
            let request = EventLoopRequest::ProgressTask(TaskHandle::new(task_id, runnable));
            proxy.send_event(request).expect("event loop should not be closed");
        };

        let (runnable, task) = async_task::spawn(future, schedule);
        runnable.schedule();

        let task = Task::new(task);

        let result = self.tasks.insert(task_id, task);
        debug_assert!(result.is_none(), "task with id {task_id} is already in tasks map");

        task_id
    }

    #[must_use]
    pub fn task_exists(&self, task_id: TaskId) -> bool {
        self.tasks.contains_key(&task_id)
    }

    pub fn try_finish_task(&mut self, task_id: TaskId) -> Option<TaskResult> {
        let task = self.tasks.get(&task_id);
        if let Some(task) = task {
            if task.is_finished() {
                return Some(self.finish_task(task_id));
            }
        }
        None
    }

    pub fn finish_task(&mut self, task_id: TaskId) -> TaskResult {
        let task = self.tasks.remove(&task_id);
        let task = task.expect("task should be in the map");
        let result = future::block_on(task.task);

        self.task_id_mask.remove_task_id(task_id);

        result
    }

    pub fn kill_task(&mut self, task_id: TaskId) {
        let task = self.tasks.remove(&task_id);
        let task = task.expect("task should be in the map");
        task.kill();

        self.task_id_mask.remove_task_id(task_id);
    }
}

pub struct TaskIdMask(BitVec);

impl TaskIdMask {
    #[must_use]
    pub fn new() -> Self {
        Self(BitVec::new())
    }

    pub fn crate_task_id(&mut self) -> TaskId {
        let free_task_id = self.0.leading_ones();
        if free_task_id == self.0.len() {
            self.0.push(true);
        } else {
            let result = self.0.replace(free_task_id, true);
            debug_assert!(!result, "task with id {free_task_id} is already in task mask");
        }
        free_task_id
    }

    /// Assumes that given `task_id` is valid
    pub fn remove_task_id(&mut self, task_id: TaskId) {
        let result = self.0.replace(task_id, false);
        debug_assert!(result, "task with id {task_id} is not in task mask");
        self.truncate();
    }

    fn truncate(&mut self) {
        let trailing_zeros = self.0.trailing_zeros();
        let length = self.0.len();
        self.0.truncate(length - trailing_zeros);
    }
}

impl Default for TaskIdMask {
    fn default() -> Self {
        Self::new()
    }
}

pub type AsyncTask = async_task::Task<TaskResult>;
pub type TaskResult = Result<u32>;
pub type TaskId = usize;

pub struct Task {
    task: AsyncTask,
}

impl Task {
    #[must_use]
    pub fn new(task: AsyncTask) -> Self {
        Self { task }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.task.is_finished()
    }

    pub fn finish(self) -> TaskResult {
        future::block_on(self.task)
    }

    pub fn kill(self) {
        drop(self.task);
    }
}

#[derive(Debug)]
pub struct TaskHandle {
    task_id: TaskId,
    runnable: Runnable,
}

impl TaskHandle {
    #[must_use]
    pub fn new(task_id: TaskId, runnable: Runnable) -> Self {
        Self { task_id, runnable }
    }

    pub fn progress(self) {
        self.runnable.run();
    }

    #[must_use]
    pub fn task_id(&self) -> TaskId {
        self.task_id
    }
}
