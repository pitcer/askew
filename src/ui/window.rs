use std::num::NonZeroU32;

use anyhow::{anyhow, Result};
use softbuffer::Context;
use winit::dpi::PhysicalSize;
use winit::window::{Window as WinitWindow, WindowId};

use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::ui::frame::panel::pixel::Pixel;

type WindowRef<'a> = &'a WinitWindow;
type Surface<'a> = softbuffer::Surface<WindowRef<'a>, WindowRef<'a>>;
type Buffer<'a, 'b> = softbuffer::Buffer<'a, WindowRef<'b>, WindowRef<'b>>;

pub struct Window<'a> {
    window: WindowRef<'a>,
    surface: Surface<'a>,
}

impl<'a> Window<'a> {
    pub fn from_winit(window: &'a WinitWindow) -> Result<Self> {
        let context = Context::new(window).map_err(|error| anyhow!(error.to_string()))?;
        let surface = Surface::new(&context, window).map_err(|error| anyhow!(error.to_string()))?;
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

    pub fn buffer_mut(&mut self) -> Result<Buffer<'_, 'a>> {
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

pub fn buffer_as_pixels<'a>(buffer: &'a mut Buffer<'_, '_>) -> &'a mut [Pixel] {
    bytemuck::cast_slice_mut(buffer)
}
