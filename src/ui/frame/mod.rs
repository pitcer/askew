use std::num::NonZeroU32;

use anyhow::{anyhow, Result};
use image::{EncodableLayout, ImageFormat, RgbImage};
use rand::Rng;
use softbuffer::{Context, Surface};
use tiny_skia::IntSize;
use tiny_skia::Pixmap;
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowId};

use crate::canvas::curve::control_points::kind::bezier::{Bezier, BezierAlgorithm};
use crate::canvas::curve::control_points::kind::interpolation::Interpolation;
use crate::canvas::curve::control_points::kind::polyline::Polyline;
use crate::canvas::curve::control_points::kind::rational_bezier::{
    RationalBezier, RationalBezierAlgorithm, RationalBezierPoint,
};
use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::ControlPointsCurveKind;
use crate::canvas::curve::formula::trochoid::Trochoid;
use crate::canvas::curve::formula::FormulaCurveKind;
use crate::canvas::curve::CurveKind;
use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::math::size::Size;
use crate::canvas::Canvas;
use crate::config::rgb::{Alpha, Rgb};
use crate::config::{Config, CurveType, SaveFormat};
use crate::event::{EventHandler, FrameEvent};
use crate::ui::command::{CommandState, MessageType};
use crate::ui::frame::event_handler::FrameEventHandler;
use crate::ui::frame::font::{FontLayout, FontLoader, GlyphRasterizer};
use crate::ui::frame::mode::Mode;
use crate::ui::frame::panel::bar::TextPanel;
use crate::ui::frame::panel::pixel::Pixel;
use crate::ui::frame::panel::Panel;

pub mod event_handler;
pub mod font;
pub mod mode;
pub mod panel;

const TEXT_COLOR: Rgb = Rgb::new(249, 250, 244);
const ERROR_COLOR: Rgb = Rgb::new(179, 26, 64);

pub struct Frame {
    window: Window,
    surface: Surface,
    canvas: Canvas,
    background: Option<Pixmap>,
    font_loader: FontLoader,
    glyph_rasterizer: GlyphRasterizer,
    status_layout: FontLayout,
    command_layout: FontLayout,
    command: CommandState,
    mode: Mode,
}

impl Frame {
    #[allow(clippy::too_many_lines)]
    pub fn new(window: Window, config: &Config) -> Result<Self> {
        let context = unsafe { Context::new(&window) }.expect("platform should be supported");
        let mut surface =
            unsafe { Surface::new(&context, &window) }.expect("platform should be supported");
        let size = window.inner_size();
        surface
            .resize(
                NonZeroU32::new(size.width).expect("size width should be non zero"),
                NonZeroU32::new(size.height).expect("size height should be non zero"),
            )
            .map_err(|error| anyhow!(error.to_string()))?;
        let background = if let Some(path) = &config.background_path {
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
            Some(image_pixmap)
        } else {
            None
        };
        let window_rectangle = Self::size_rectangle(size);
        let canvas_rectangle: Rectangle<f32> = window_rectangle.into();
        let mut rng = rand::thread_rng();
        let points_vec = (0..config.random_points)
            .map(|_| {
                Point::new(
                    rng.gen_range(
                        canvas_rectangle.origin().horizontal()..=canvas_rectangle.size().width(),
                    ),
                    rng.gen_range(
                        canvas_rectangle.origin().vertical()..=canvas_rectangle.size().height(),
                    ),
                )
            })
            .collect::<Vec<_>>();
        let points = ControlPoints::new(points_vec);
        let canvas = match config.curve_type {
            CurveType::Polyline => Canvas::new(
                canvas_rectangle,
                vec![CurveKind::ControlPoints(ControlPointsCurveKind::Polyline(
                    Polyline::new(points),
                ))],
                config,
            ),
            CurveType::Interpolation => Canvas::new(
                canvas_rectangle,
                vec![CurveKind::ControlPoints(
                    ControlPointsCurveKind::Interpolation(Interpolation::new(
                        points,
                        config.samples,
                        config.chebyshev_nodes,
                    )),
                )],
                config,
            ),
            CurveType::Bezier => Canvas::new(
                canvas_rectangle,
                vec![CurveKind::ControlPoints(ControlPointsCurveKind::Bezier(
                    Bezier::new(points, config.samples, BezierAlgorithm::ChudyWozny),
                ))],
                config,
            ),
            CurveType::RationalBezier => {
                let points = (0..config.random_points)
                    .map(|_| {
                        RationalBezierPoint::new(
                            Point::new(
                                rng.gen_range(
                                    canvas_rectangle.origin().horizontal()
                                        ..=canvas_rectangle.size().width(),
                                ),
                                rng.gen_range(
                                    canvas_rectangle.origin().vertical()
                                        ..=canvas_rectangle.size().height(),
                                ),
                            ),
                            rng.gen_range(0.0..1.0),
                        )
                    })
                    .collect::<Vec<_>>();
                let points = ControlPoints::new(points);
                Canvas::new(
                    canvas_rectangle,
                    vec![CurveKind::ControlPoints(
                        ControlPointsCurveKind::RationalBezier(RationalBezier::new(
                            points,
                            config.samples,
                            RationalBezierAlgorithm::ChudyWozny,
                        )),
                    )],
                    config,
                )
            }
            CurveType::Trochoid => Canvas::new(
                Rectangle::new(Point::new(-2.0, -2.0), Size::new(4.0, 4.0)),
                vec![CurveKind::Formula(FormulaCurveKind::Trochoid(
                    Trochoid::new(
                        5000,
                        (10.0 * -std::f32::consts::PI, 10.0 * std::f32::consts::PI),
                        0.3,
                        0.8,
                        0.3,
                        0.7,
                    ),
                ))],
                config,
            ),
        };
        let font_loader = FontLoader::new(&config.font_path)?;
        let glyph_rasterizer = GlyphRasterizer::new();
        let status_layout = FontLayout::new(config.font_size);
        let command_layout = FontLayout::new(config.font_size);
        let command = CommandState::initial();
        let mode = Mode::Normal;
        Ok(Self {
            window,
            surface,
            canvas,
            background,
            font_loader,
            glyph_rasterizer,
            status_layout,
            command_layout,
            command,
            mode,
        })
    }

    fn size_rectangle(size: PhysicalSize<u32>) -> Rectangle<u32> {
        let origin = Point::new(0, 0);
        let size = size.into();
        Rectangle::new(origin, size)
    }

    pub fn event_handler(&mut self) -> FrameEventHandler<'_> {
        FrameEventHandler::new(self)
    }

    pub fn receive_event(&mut self, event: FrameEvent) -> Result<()> {
        log::debug!("Event received from input: {event:?}");

        let command_closed = self.command.is_closed();
        let mut handler = self.event_handler();
        // TODO: replace with state machine
        match event {
            FrameEvent::ToggleConvexHull(event) if command_closed => handler.handle(event)?,
            FrameEvent::ChangeWeight(event) if command_closed => handler.handle(event)?,
            FrameEvent::MovePoint(event) if command_closed => handler.handle(event)?,
            FrameEvent::AddPoint(event) if command_closed => handler.handle(event)?,
            FrameEvent::AddCurve(event) if command_closed => handler.handle(event)?,
            FrameEvent::Delete(event) if command_closed => handler.handle(event)?,
            FrameEvent::ChangeIndex(event) if command_closed => handler.handle(event)?,
            FrameEvent::ChangeMode(event) if command_closed => handler.handle(event)?,

            FrameEvent::EnterCommand(event) => handler.handle(event)?,
            FrameEvent::ReceiveCharacter(event) => handler.handle(event)?,
            FrameEvent::ExecuteCommand(event) => handler.handle(event)?,
            FrameEvent::ExitMode(event) => handler.handle(event)?,

            FrameEvent::ToggleConvexHull(_)
            | FrameEvent::ChangeWeight(_)
            | FrameEvent::MovePoint(_)
            | FrameEvent::AddPoint(_)
            | FrameEvent::AddCurve(_)
            | FrameEvent::Delete(_)
            | FrameEvent::ChangeIndex(_)
            | FrameEvent::ChangeMode(_) => {}
        }

        self.window.request_redraw();

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let mut buffer = self
            .surface
            .buffer_mut()
            .map_err(|error| anyhow!(error.to_string()))?;
        let area = Self::size_rectangle(self.window.inner_size());
        let mut panel = Panel::from_buffer(&mut buffer, area);

        panel.fill(Pixel::from_rgba(Rgb::new(32, 32, 32), Alpha::max()));
        if let Some(background) = &self.background {
            panel.draw_pixmap(0, 0, background.as_ref());
        }
        let size = area.size();
        let split_layout = [size.height() as usize - 44, 22, 22];
        let [panel, status, command] = panel.split_vertical(split_layout);

        let mut name = self.canvas.curves()[self.canvas.properties().current_curve].to_string();
        name.truncate(4);
        self.status_layout
            .setup(&self.font_loader)
            .append_text(&format!(
                "{} {} {}/{} {}",
                self.mode,
                name,
                self.canvas.properties().current_curve + 1,
                self.canvas.curves().len(),
                self.canvas.properties().current_point_index
            ));
        let mut status_bar = TextPanel::new(status, TEXT_COLOR, Rgb::new(42, 42, 42));
        status_bar.fill();
        status_bar.draw_layout(
            &self.font_loader,
            &self.status_layout,
            &mut self.glyph_rasterizer,
        );

        let mut setup = self.command_layout.setup(&self.font_loader);
        match &self.command {
            CommandState::Closed(command) => {
                if let Some(message) = command.message() {
                    let color = Self::message_color(message.message_type());
                    setup.append_color_text(message.message(), color);
                } else {
                    setup.append_text(" ");
                }
            }
            CommandState::Open(command) => {
                let buffer = command.input();
                setup.append_color_text(buffer, TEXT_COLOR);
                setup.append_color_text("█", TEXT_COLOR);
            }
        }
        let mut command_bar = TextPanel::new(command, TEXT_COLOR, Rgb::new(42, 42, 42));
        command_bar.fill();
        command_bar.draw_layout(
            &self.font_loader,
            &self.command_layout,
            &mut self.glyph_rasterizer,
        );

        self.canvas.rasterize(panel)?;

        buffer
            .present()
            .map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }

    fn message_color(message_type: &MessageType) -> Rgb {
        match message_type {
            MessageType::Info => TEXT_COLOR,
            MessageType::Error => ERROR_COLOR,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        self.surface
            .resize(
                NonZeroU32::new(size.width).expect("size width should be non zero"),
                NonZeroU32::new(size.height).expect("size height should be non zero"),
            )
            .map_err(|error| anyhow!(error.to_string()))
    }

    pub fn save(&mut self, format: SaveFormat) -> Result<()> {
        match format {
            SaveFormat::Png => {
                const EMPTY_PIXEL: Pixel = Pixel::from_rgba(Rgb::new(0, 0, 0), Alpha::min());
                let area = Self::size_rectangle(self.window.inner_size());
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

    #[must_use]
    pub fn has_id(&self, id: WindowId) -> bool {
        self.window.id() == id
    }
}
