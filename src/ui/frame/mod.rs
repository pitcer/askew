use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use image::{EncodableLayout, RgbImage};
use tiny_skia::IntSize;
use tiny_skia::Pixmap;

use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::Canvas;
use crate::config::{CanvasConfig, FrameConfig};
use crate::ui::frame::event_handler::CommandEventHandler;
use crate::ui::frame::panel::pixel::Pixel;
use crate::ui::frame::panel::Panel;
use crate::ui::frame::properties::FrameProperties;
use crate::ui::mode::ModeState;

pub mod event_handler;
pub mod panel;
pub mod properties;

#[derive(Debug)]
pub struct Frame {
    canvas: Canvas,
    size: Rectangle<u32>,
    properties: FrameProperties,
    background: Option<Pixmap>,
}

impl Frame {
    pub fn new(
        size: Rectangle<u32>,
        frame_config: FrameConfig,
        canvas_config: CanvasConfig,
    ) -> Result<Self> {
        let background =
            frame_config.background_to_load_path.as_ref().map(Self::load_background).transpose()?;

        let mut canvas = frame_config
            .project_to_open_path
            .as_ref()
            .map_or_else(|| Ok(Canvas::new(size.into(), canvas_config)), Canvas::from_file)?;

        if frame_config.generate_random_points > 0 {
            canvas.generate_random_points(frame_config.generate_random_points)?;
        }

        let properties = FrameProperties::new(frame_config);

        Ok(Self { canvas, size, properties, background })
    }

    fn load_background(path: impl AsRef<Path>) -> Result<Pixmap> {
        let image = image::open(path)?;
        let image = image.into_rgb8();
        let buffer: &[[u8; 3]] = bytemuck::cast_slice(image.as_bytes());
        let buffer =
            buffer.iter().copied().flat_map(|[r, g, b]| [b, g, r, 255]).collect::<Vec<_>>();
        let image_pixmap =
            Pixmap::from_vec(buffer, IntSize::from_wh(image.width(), image.height()).unwrap())
                .unwrap();
        Ok(image_pixmap)
    }

    pub fn resize(&mut self, size: Rectangle<u32>) {
        self.canvas.resize(size.into());
        self.size = size;
    }

    pub fn event_handler<'a>(&'a mut self, mode: &'a mut ModeState) -> CommandEventHandler<'a> {
        CommandEventHandler::new(self, mode)
    }

    pub fn handle_close(&mut self) -> Result<()> {
        Ok(())
    }

    /// If path is `None`, then default image save path from config will be used.
    pub fn save_image<'a, P>(&'a self, path: Option<P>) -> Result<P>
    where
        P: AsRef<Path> + From<&'a PathBuf>,
    {
        let mut buffer = vec![Pixel::default(); self.size.area() as usize];
        let mut panel = Panel::new(&mut buffer, self.size);
        self.canvas.draw_on_all(&mut panel.as_pixmap_mut());
        let buffer = buffer.iter().flat_map(|pixel| pixel.into_rgb_array()).collect::<Vec<_>>();
        let size = self.size.size();
        let image = RgbImage::from_raw(size.width(), size.height(), buffer)
            .ok_or_else(|| anyhow!("image should fit"))?;
        let path = path.unwrap_or_else(|| P::from(&self.properties.default_image_save_path));
        image.save(&path)?;
        Ok(path)
    }

    pub fn save_canvas<'a, P>(&'a self, path: Option<P>) -> Result<P>
    where
        P: AsRef<Path> + From<&'a PathBuf>,
    {
        let path = path.unwrap_or_else(|| P::from(&self.properties.default_project_save_path));
        self.canvas.save_to_file(&path)?;
        Ok(path)
    }

    pub fn open_canvas<'a, P>(&'a self, path: Option<P>) -> Result<(P, Canvas)>
    where
        P: AsRef<Path> + From<&'a PathBuf>,
    {
        let path = path.unwrap_or_else(|| P::from(&self.properties.default_project_save_path));
        let canvas = Canvas::from_file(&path)?;
        Ok((path, canvas))
    }

    pub fn load_canvas(&mut self, mut canvas: Canvas) {
        let size = self.size.into();
        canvas.resize(size);
        self.canvas = canvas;
    }

    #[must_use]
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    #[must_use]
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    #[must_use]
    pub fn background(&self) -> &Option<Pixmap> {
        &self.background
    }

    #[must_use]
    pub fn properties(&self) -> &FrameProperties {
        &self.properties
    }
}
