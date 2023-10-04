use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

use anyhow::Result;
use async_channel::Sender;

use crate::wasm::request::Response;

pub struct SleepingTasks {
    tasks: BinaryHeap<Reverse<SleepingTask>>,
}

impl SleepingTasks {
    #[must_use]
    pub fn new() -> Self {
        let tasks = BinaryHeap::new();
        Self { tasks }
    }

    /// Returns new earliest wake time
    pub fn sleep(&mut self, waker: Sender<Response>, duration: Duration) -> Instant {
        let now = Instant::now();
        let until = now + duration;
        self.sleep_until(waker, until)
    }

    fn sleep_until(&mut self, waker: Sender<Response>, until: Instant) -> Instant {
        let task = SleepingTask::new(waker, until);
        let task = Reverse(task);
        self.tasks.push(task);
        self.earliest_wake_time()
            .expect("wake time should exist, because entry was just pushed to the heap")
    }

    /// Returns new earliest wake time or None if there is no sleeping task
    pub fn wake(&mut self, wake_time: Instant) -> Result<Option<Instant>> {
        let drain =
            DrainFilterSorted::new(&mut self.tasks, |Reverse(task)| task.wake_time <= wake_time);
        for Reverse(task) in drain {
            task.wake()?;
        }
        Ok(self.earliest_wake_time())
    }

    #[must_use]
    fn earliest_wake_time(&self) -> Option<Instant> {
        let Reverse(task) = self.tasks.peek()?;
        Some(task.wake_time())
    }
}

impl Default for SleepingTasks {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SleepingTask {
    waker: Sender<Response>,
    wake_time: Instant,
}

impl SleepingTask {
    #[must_use]
    pub fn new(waker: Sender<Response>, wake_time: Instant) -> Self {
        Self { waker, wake_time }
    }

    pub fn wake(self) -> Result<()> {
        self.waker.send_blocking(Response::Sleep)?;
        Ok(())
    }

    #[must_use]
    pub fn wake_time(&self) -> Instant {
        self.wake_time
    }
}

impl PartialEq for SleepingTask {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.wake_time, &other.wake_time)
    }
}

impl Eq for SleepingTask {}

impl PartialOrd for SleepingTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for SleepingTask {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.wake_time, &other.wake_time)
    }
}

struct DrainFilterSorted<'a, T, F> {
    heap: &'a mut BinaryHeap<T>,
    predicate: F,
}

impl<'a, T, F> DrainFilterSorted<'a, T, F>
where
    F: Fn(&T) -> bool,
{
    pub fn new(heap: &'a mut BinaryHeap<T>, predicate: F) -> Self {
        Self { heap, predicate }
    }
}

impl<'a, T, F> Iterator for DrainFilterSorted<'a, T, F>
where
    T: Ord,
    F: Fn(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.heap.peek()?;
        if (self.predicate)(element) {
            self.heap.pop()
        } else {
            None
        }
    }
}
