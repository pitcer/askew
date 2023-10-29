use std::num::NonZeroU32;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use softbuffer::Context;
use winit::dpi::PhysicalSize;
use winit::window::{Window as WinitWindow, WindowId};

use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;

type Surface = softbuffer::Surface<Rc<WinitWindow>, Rc<WinitWindow>>;
pub type Buffer<'a> = softbuffer::Buffer<'a, Rc<WinitWindow>, Rc<WinitWindow>>;

pub struct Window {
    window: Rc<WinitWindow>,
    surface: Surface,
}

impl Window {
    pub fn from_winit(window: WinitWindow) -> Result<Self> {
        let window = Rc::new(window);
        let context = Context::new(Rc::clone(&window)).expect("platform should be supported");
        let surface =
            Surface::new(&context, Rc::clone(&window)).expect("platform should be supported");
        let size = window.inner_size();
        let mut window = Self { window, surface };
        window.resize_surface(size)?;
        Ok(window)
    }

    pub fn resize_surface(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        let width = NonZeroU32::new(size.width).expect("size width should be non zero");
        let height = NonZeroU32::new(size.height).expect("size height should be non zero");
        self.surface.resize(width, height).map_err(|error| anyhow!(error.to_string()))
    }

    pub fn buffer_mut(&mut self) -> Result<Buffer<'_>> {
        self.surface.buffer_mut().map_err(|error| anyhow!(error.to_string()))
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    #[must_use]
    pub fn size_rectangle(&self) -> Rectangle<u32> {
        let size = self.window.inner_size();
        let origin = Point::new(0, 0);
        let size = size.into();
        Rectangle::new(origin, size)
    }

    #[must_use]
    pub fn has_id(&self, id: WindowId) -> bool {
        self.window.id() == id
    }
}
