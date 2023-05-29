use anyhow::Result;
use clap::Parser;
use winit::event_loop::EventLoop;

use askew::command::Command;
use askew::ui::event_handler::EventHandler;
use askew::ui::frame::Frame;

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
