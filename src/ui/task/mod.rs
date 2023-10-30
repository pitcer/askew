use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use anyhow::Result;
use async_task::Runnable;
use bitvec::vec::BitVec;
use futures_lite::future;

use crate::executor;
use crate::ui::handler::message::{HandlerMessage, RunnerSender};
use crate::ui::task::lock::TaskLock;
use crate::wasm::wit::RunResult;
use crate::wasm::WasmRuntime;

pub mod lock;

pub struct Tasks {
    tasks: HashMap<TaskId, Task>,
    task_id_mask: TaskIdMask,
    runtime: WasmRuntime,
    runner: RunnerSender,
    lock: TaskLock,
}

impl Tasks {
    pub fn new(runner: RunnerSender) -> Result<Self> {
        let tasks = HashMap::new();
        let runtime = WasmRuntime::new()?;
        let task_id_mask = TaskIdMask::new();
        let lock = TaskLock::new()?;
        Ok(Self { tasks, task_id_mask, runtime, runner, lock })
    }

    pub fn list_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.values()
    }

    pub fn register_task(&mut self, path: PathBuf, argument: Option<String>) -> Result<TaskId> {
        let task_id = self.task_id_mask.crate_task_id();
        let proxy = RunnerSender::clone(&self.runner);
        let lock = TaskLock::clone(&self.lock);
        let wasm_task = self.runtime.create_task(&path, task_id, proxy, lock)?;

        let sender = RunnerSender::clone(&self.runner);
        let future = async move {
            let result = wasm_task.run(argument).await;
            sender.send_event(HandlerMessage::TaskFinished(task_id))?;
            result
        };
        let task = executor::spawn(future);
        let task = Task::new(task, task_id, path);

        let result = self.tasks.insert(task_id, task);
        debug_assert!(result.is_none(), "task with id {task_id} is already in tasks map");

        Ok(task_id)
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
        let result = task.finish();

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
pub type TaskResult = Result<RunResult>;
pub type TaskId = usize;

#[derive(Debug)]
pub struct Task {
    task: AsyncTask,
    id: TaskId,
    path: PathBuf,
}

impl Task {
    #[must_use]
    pub fn new(task: AsyncTask, id: TaskId, path: PathBuf) -> Self {
        Self { task, id, path }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.task.is_finished()
    }

    /// Blocks on task's future
    pub fn finish(self) -> TaskResult {
        future::block_on(self.task)
    }

    pub fn kill(self) {
        drop(self.task);
    }

    #[must_use]
    pub fn id(&self) -> TaskId {
        self.id
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Display for Task {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let file_name = self.path.file_name().unwrap_or("<invalid>".as_ref()).to_string_lossy();
        write!(formatter, "{}: '{file_name}'", self.id)
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
