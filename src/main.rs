use anyhow::Result;
use winit::event_loop::EventLoop;

use crate::ui::event_handler::EventHandler;
use crate::ui::frame::Frame;

mod canvas;
mod ui;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let frame = Frame::new(&event_loop)?;
    let mut handler = EventHandler::new(frame);
    event_loop.run(move |event, _, control_flow| {
        let result = handler.run(event, control_flow);
        result.expect("Error in event loop")
    });
}
