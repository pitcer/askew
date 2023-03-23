use anyhow::Result;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    let mut context = unsafe { softbuffer::GraphicsContext::new(&window, &window) }
        .expect("Platform is not supported");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let size = window.inner_size();
                let (width, height) = (size.width, size.height);
                let buffer = vec![0xffffffff; (width * height) as usize];
                context.set_buffer(&buffer, width as u16, height as u16);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                control_flow.set_exit();
            }
            _ => {}
        }
    });
}
