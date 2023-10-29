use std::time::Duration;

use anyhow::Result;
use async_io::Async;
use winit::event_loop::ControlFlow;
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

use crate::ui::handler::message::HandlerMessage;
use crate::ui::handler::WindowHandler;

pub mod task;

type EventLoop = winit::event_loop::EventLoop<HandlerMessage>;

pub async fn run(
    event_loop: EventLoop,
    event_handler: &mut WindowHandler<'_>,
) -> Result<RunnerExitResult> {
    // Prevents blocking on `pump_events`.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut async_event_loop = Async::new(event_loop)?;

    let exit_code = loop {
        // TODO: try to do something with this (maybe winit would fix this someday):
        // There is a bug on Wayland, that if you hold some key, only the first event will trigger
        // readability of event loop. Also those next events won't accumulate in event queue in the
        // event loop. The following warning would be shown after pulling the key:
        // `[calloop] Received an event for non-existence source: TokenInner`
        async_event_loop.readable().await?;

        // SAFETY: event_loop I/O is not dropped
        let status = unsafe {
            let event_loop = async_event_loop.get_mut();
            event_loop.pump_events(Some(Duration::ZERO), |event, target| {
                event_handler.handle(event, target);
            })
        };

        if let PumpStatus::Exit(exit_code) = status {
            break exit_code;
        }
    };

    // For some reason it's necessary to return event_loop, because otherwise we get segfault.
    let event_loop = async_event_loop.into_inner()?;
    Ok(RunnerExitResult::new(exit_code, event_loop))
}

pub struct RunnerExitResult {
    exit_code: i32,
    event_loop: EventLoop,
}

impl RunnerExitResult {
    #[must_use]
    pub fn new(exit_code: i32, event_loop: EventLoop) -> Self {
        Self { exit_code, event_loop }
    }

    #[must_use]
    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    #[must_use]
    pub fn into_event_loop(self) -> EventLoop {
        self.event_loop
    }
}
