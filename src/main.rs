use anyhow::Result;
use clap::Parser;
use winit::event_loop::EventLoop;

use crate::command::Command;
use crate::ui::event_handler::EventHandler;
use crate::ui::frame::Frame;

mod canvas;
mod command;
mod ui;

fn main() -> Result<()> {
    let command = Command::parse();
    let event_loop = EventLoop::new();
    let frame = Frame::new(&event_loop, &command)?;
    let mut handler = EventHandler::new(frame, &command);
    event_loop.run(move |event, _, control_flow| {
        let result = handler.run(event, control_flow);
        result.expect("Error in event loop")
    });
}
