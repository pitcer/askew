use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use anyhow::Result;
use async_task::Runnable;
use futures_lite::future;

use crate::executor;
use crate::id_assigner::IdAssigner;
use crate::ui::handler::message::{HandlerMessage, HandlerSender};
use crate::ui::shared::SharedFrame;
use crate::ui::task::lock::TaskLock;
use crate::wasm::WasmRuntime;

pub mod lock;

#[derive(Debug)]
pub struct Tasks {
    tasks: HashMap<TaskId, Task>,
    task_id_mask: IdAssigner,
    runtime: WasmRuntime,
    sender: HandlerSender,
    frame: SharedFrame,
    lock: TaskLock,
}

impl Tasks {
    pub fn new(sender: HandlerSender, frame: SharedFrame) -> Result<Self> {
        let tasks = HashMap::new();
        let runtime = WasmRuntime::new()?;
        let task_id_mask = IdAssigner::new();
        let lock = TaskLock::new()?;
        Ok(Self { tasks, task_id_mask, runtime, sender, frame, lock })
    }

    pub fn list_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.values()
    }

    pub fn register_task(&mut self, path: PathBuf, argument: Option<String>) -> Result<TaskId> {
        let task_id = self.task_id_mask.assign_id();
        let proxy = HandlerSender::clone(&self.sender);
        let frame = SharedFrame::clone(&self.frame);
        let lock = TaskLock::clone(&self.lock);
        let wasm_task = self.runtime.create_task(&path, proxy, frame, lock)?;

        let sender = HandlerSender::clone(&self.sender);
        let future = async move {
            let result = wasm_task.run(argument).await?;
            sender.send_event(HandlerMessage::TaskFinished(task_id, result))?;
            Ok(())
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

        self.task_id_mask.remove_id(task_id);

        result
    }

    pub fn kill_task(&mut self, task_id: TaskId) {
        let task = self.tasks.remove(&task_id);
        let task = task.expect("task should be in the map");
        task.kill();

        self.task_id_mask.remove_id(task_id);
    }
}

pub type AsyncTask = async_task::Task<TaskResult>;
pub type TaskResult = Result<()>;
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
