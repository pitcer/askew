use std::fs::File;
use std::path::Path;

use anyhow::{anyhow, Result};
use image::{EncodableLayout, ImageFormat, RgbImage};
use tiny_skia::IntSize;
use tiny_skia::Pixmap;

use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::Canvas;
use crate::config::rgb::{Alpha, Rgb};
use crate::config::{Config, SaveFormat};
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
    pub fn new(size: Rectangle<u32>, config: &Config) -> Result<Self> {
        let background = Self::load_background(config)?;

        let mut canvas = Canvas::new(size.into(), config);
        let properties = FrameProperties::new(config);

        if config.random_points > 0 {
            canvas.generate_random_points(config.random_points)?;
        }

        Ok(Self {
            canvas,
            size,
            properties,
            background,
        })
    }

    fn load_background(config: &Config) -> Result<Option<Pixmap>> {
        if let Some(path) = &config.background_path {
            let image = image::open(path)?;
            let image = image.into_rgb8();
            let buffer: &[[u8; 3]] = bytemuck::cast_slice(image.as_bytes());
            let buffer = buffer
                .iter()
                .copied()
                .flat_map(|[r, g, b]| [b, g, r, 255])
                .collect::<Vec<_>>();
            let image_pixmap = Pixmap::from_vec(
                buffer,
                IntSize::from_wh(image.width(), image.height()).unwrap(),
            )
            .unwrap();
            Ok(Some(image_pixmap))
        } else {
            Ok(None)
        }
    }

    pub fn resize(&mut self, size: Rectangle<u32>) {
        self.canvas.resize(size.into());
        self.size = size;
    }

    pub fn event_handler<'a>(&'a mut self, mode: &'a mut ModeState) -> CommandEventHandler<'a> {
        CommandEventHandler::new(self, mode)
    }

    pub fn handle_close(&mut self) -> Result<()> {
        if let Some(format) = self.properties.save_format {
            self.save_image(format)?;
        }
        Ok(())
    }

    pub fn save_image(&self, format: SaveFormat) -> Result<()> {
        match format {
            SaveFormat::Png => {
                const EMPTY_PIXEL: Pixel = Pixel::from_rgba(Rgb::new(0, 0, 0), Alpha::min());
                let mut buffer = vec![EMPTY_PIXEL; self.size.area() as usize];
                let panel = Panel::new(&mut buffer, self.size);
                self.canvas.rasterize(panel)?;
                let buffer = buffer
                    .iter()
                    .flat_map(|pixel| pixel.into_rgb_array())
                    .collect::<Vec<_>>();
                let size = self.size.size();
                let image = RgbImage::from_raw(size.width(), size.height(), buffer)
                    .ok_or_else(|| anyhow!("image should fit"))?;
                image.save_with_format("curve.png", ImageFormat::Png)?;
            }
        }
        Ok(())
    }

    pub fn save_canvas(&self, path: impl AsRef<Path>) -> Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, &self.canvas)?;
        Ok(())
    }

    pub fn open_canvas(path: impl AsRef<Path>) -> Result<Canvas> {
        let file = File::open(path)?;
        let canvas = serde_json::from_reader(file)?;
        Ok(canvas)
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
    pub fn background(&self) -> &Option<Pixmap> {
        &self.background
    }

    #[must_use]
    pub fn properties(&self) -> &FrameProperties {
        &self.properties
    }
}
