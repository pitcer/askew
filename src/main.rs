use anyhow::Result;
use clap::Parser;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use askew::command::Command;
use askew::ui::event_handler::EventHandler;
use askew::ui::frame::Frame;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let command = Command::parse();
    let window = WindowBuilder::new()
        .with_title("askew")
        .build(&event_loop)?;
    let frame = Frame::new(window, &command)?;
    let mut handler = EventHandler::new(frame, &command);
    event_loop.run(move |event, _, control_flow| {
        let result = handler.run(event, control_flow);
        result.expect("Error in event loop")
    });
}
