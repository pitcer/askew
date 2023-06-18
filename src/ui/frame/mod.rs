use anyhow::{anyhow, Result};
use image::{EncodableLayout, ImageFormat, RgbImage};
use rand::Rng;
use tiny_skia::IntSize;
use tiny_skia::Pixmap;
use winit::dpi::PhysicalSize;
use winit::window::{Window as WinitWindow, WindowId};

use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::Canvas;
use crate::config::rgb::{Alpha, Rgb};
use crate::config::{Config, SaveFormat};
use crate::event::canvas::AddPoint;
use crate::event::{EventHandler, InputEvent};
use crate::ui::command::CommandState;
use crate::ui::frame::drawer::Drawer;
use crate::ui::frame::event_handler::InputEventHandler;
use crate::ui::frame::mode::ModeState;
use crate::ui::frame::panel::pixel::Pixel;
use crate::ui::frame::panel::Panel;
use crate::ui::frame::view::FrameView;
use crate::ui::window::Window;

pub mod drawer;
pub mod event_handler;
pub mod font;
pub mod mode;
pub mod panel;
pub mod view;

pub struct Frame {
    window: Window,
    canvas: Canvas,
    background: Option<Pixmap>,
    command: CommandState,
    mode: ModeState,
    drawer: Drawer,
}

impl Frame {
    pub fn new(window: WinitWindow, config: &Config) -> Result<Self> {
        let window = Window::from_winit(window)?;
        let background = Self::load_background(config)?;

        let window_rectangle = window.size_rectangle();
        let canvas_rectangle: Rectangle<f32> = window_rectangle.into();
        let mut canvas = Canvas::new(canvas_rectangle, config);

        if config.random_points > 0 {
            Self::generate_random_points(&mut canvas, config.random_points)?;
        }

        let command = CommandState::initial();
        let mode = ModeState::initial();
        let drawer = Drawer::new(config)?;

        Ok(Self {
            window,
            canvas,
            background,
            command,
            mode,
            drawer,
        })
    }

    fn generate_random_points(canvas: &mut Canvas, number_of_points: u32) -> Result<()> {
        let mut random = rand::thread_rng();

        let canvas_rectangle = canvas.properties().area;
        let origin = canvas_rectangle.origin();
        let size = canvas_rectangle.size();

        for _ in 0..number_of_points {
            let horizontal = random.gen_range(origin.horizontal()..=size.width());
            let vertical = random.gen_range(origin.vertical()..=size.height());
            let point = Point::new(horizontal, vertical);
            canvas.event_handler().handle(AddPoint::new(point))?;
        }
        Ok(())
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

    pub fn event_handler(&mut self) -> InputEventHandler<'_> {
        InputEventHandler::new(self)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        self.window.resize_surface(size)
    }

    pub fn receive_event(&mut self, event: InputEvent) -> Result<()> {
        log::debug!("Event received from input: {event:?}");

        let command_closed = self.command.is_closed();
        let mut handler = self.event_handler();
        match event {
            InputEvent::ToggleConvexHull(event) if command_closed => handler.handle(event)?,
            InputEvent::ChangeWeight(event) if command_closed => handler.handle(event)?,
            InputEvent::MovePoint(event) if command_closed => handler.handle(event)?,
            InputEvent::MouseClick(event) if command_closed => handler.handle(event)?,
            InputEvent::AddCurve(event) if command_closed => handler.handle(event)?,
            InputEvent::Delete(event) if command_closed => handler.handle(event)?,
            InputEvent::ChangeIndex(event) if command_closed => handler.handle(event)?,
            InputEvent::ChangeMode(event) if command_closed => handler.handle(event)?,

            InputEvent::EnterCommand(event) => handler.handle(event)?,
            InputEvent::ReceiveCharacter(event) => handler.handle(event)?,
            InputEvent::ExecuteCommand(event) => handler.handle(event)?,
            InputEvent::ExitMode(event) => handler.handle(event)?,

            InputEvent::ToggleConvexHull(_)
            | InputEvent::ChangeWeight(_)
            | InputEvent::MovePoint(_)
            | InputEvent::MouseClick(_)
            | InputEvent::AddCurve(_)
            | InputEvent::Delete(_)
            | InputEvent::ChangeIndex(_)
            | InputEvent::ChangeMode(_) => {}
        }

        self.window.request_redraw();

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let area = self.window.size_rectangle();
        let mut buffer = self.window.buffer_mut()?;
        let panel = Panel::from_buffer(&mut buffer, area);
        let view = FrameView::new(
            &mut self.canvas,
            &self.background,
            &self.command,
            &self.mode,
        );

        self.drawer.draw(view, panel)?;

        buffer
            .present()
            .map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }

    pub fn save(&mut self, format: SaveFormat) -> Result<()> {
        match format {
            SaveFormat::Png => {
                const EMPTY_PIXEL: Pixel = Pixel::from_rgba(Rgb::new(0, 0, 0), Alpha::min());
                let area = self.window.size_rectangle();
                let mut buffer = vec![EMPTY_PIXEL; area.area() as usize];
                let panel = Panel::new(&mut buffer, area);
                self.canvas.rasterize(panel)?;
                let buffer = buffer
                    .iter()
                    .flat_map(|pixel| pixel.into_rgb_array())
                    .collect::<Vec<_>>();
                let size = area.size();
                let image = RgbImage::from_raw(size.width(), size.height(), buffer)
                    .ok_or_else(|| anyhow!("image should fit"))?;
                image.save_with_format("curve.png", ImageFormat::Png)?;
            }
        }
        Ok(())
    }

    pub fn has_id(&self, id: WindowId) -> bool {
        self.window.has_id(id)
    }
}
