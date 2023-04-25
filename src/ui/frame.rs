use anyhow::Result;
use softbuffer::GraphicsContext;
use tiny_skia::Pixmap;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder, WindowId};

use crate::canvas::curve::interpolation::Interpolation;
use crate::canvas::curve::Curve;
use crate::canvas::geometry::point::Point;
use crate::canvas::geometry::rectangle::Rectangle;
use crate::canvas::layout::Layout;
use crate::canvas::paint::BgraColor;
use crate::canvas::Canvas;

pub struct Frame {
    window: Window,
    context: GraphicsContext,
    layout: Layout,
    canvas: Canvas,
}

impl Frame {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let window = WindowBuilder::new().with_title("cue").build(event_loop)?;
        let context =
            unsafe { GraphicsContext::new(&window, &window) }.expect("Platform is not supported");
        let size = window.inner_size();
        let pixmap = Pixmap::new(size.width, size.height).expect("Size should be valid");
        let window_rectangle = Self::size_rectangle(size);
        let layout = Layout::new(pixmap, window_rectangle);
        // let canvas = Canvas::new(
        //     Rectangle::new(Point::new(-2.0, -2.0), Size::new(4.0, 4.0)),
        //     Curve::Trochoid(Trochoid::new(
        //         5000,
        //         (10.0 * -std::f32::consts::PI, 10.0 * std::f32::consts::PI),
        //         0.3,
        //         0.8,
        //         0.3,
        //         0.7,
        //     )),
        // );
        // let canvas = Canvas::new(
        //     window_rectangle.into(),
        //     Curve::Polyline(Polyline::new(Vec::new())),
        // );
        let canvas = Canvas::new(
            window_rectangle.into(),
            Curve::Interpolation(Interpolation::new(Vec::new(), 5000)),
        );
        Ok(Self {
            window,
            context,
            layout,
            canvas,
        })
    }

    fn size_rectangle(size: PhysicalSize<u32>) -> Rectangle<u32> {
        let origin = Point::new(0, 0);
        let size = size.into();
        Rectangle::new(origin, size)
    }

    pub fn draw(&mut self) -> Result<()> {
        self.layout.fill(BgraColor::from_rgba(32, 32, 32, 255));
        let panel = self.layout.panel();
        self.canvas.rasterize(panel)?;
        let buffer = self.layout.buffer();
        let buffer = buffer.data();
        let buffer = bytemuck::cast_slice(buffer);
        let size = self.layout.area().size();
        self.context
            .set_buffer(buffer, size.width() as u16, size.height() as u16);
        Ok(())
    }

    pub fn add_point(&mut self, position: PhysicalPosition<f64>) {
        let point = self.scale_position(position);
        self.canvas.add_point(point);
        self.window.request_redraw();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let pixmap = Pixmap::new(size.width, size.height).expect("Size should be valid");
        let rectangle = Self::size_rectangle(size);
        let layout = Layout::new(pixmap, rectangle);
        self.layout = layout
    }

    pub fn has_id(&self, id: WindowId) -> bool {
        self.window.id() == id
    }

    fn scale_position(&self, position: PhysicalPosition<f64>) -> Point<f32> {
        Point::new(position.x as f32, position.y as f32)
    }
}
