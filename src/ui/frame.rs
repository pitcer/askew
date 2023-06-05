use anyhow::{anyhow, Result};
use image::{EncodableLayout, ImageFormat, RgbImage};
use rand::Rng;
use softbuffer::GraphicsContext;
use tiny_skia::IntSize;
use tiny_skia::Pixmap;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder, WindowId};

use crate::bar::Bar;
use crate::canvas::curve::control_points::bezier::{Bezier, BezierAlgorithm};
use crate::canvas::curve::control_points::interpolation::Interpolation;
use crate::canvas::curve::control_points::polyline::Polyline;
use crate::canvas::curve::control_points::rational_bezier::{
    RationalBezier, RationalBezierAlgorithm, RationalBezierPoint,
};
use crate::canvas::curve::control_points::{ControlPoints, ControlPointsCurve};
use crate::canvas::curve::formula::trochoid::Trochoid;
use crate::canvas::curve::formula::FormulaCurve;
use crate::canvas::curve::Curve;
use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::math::size::Size;
use crate::canvas::Canvas;
use crate::command::{Command, CurveType, SaveFormat};
use crate::event::CanvasEvent;
use crate::ui::paint::PaintColor;
use crate::ui::panel::Panel;

pub struct Frame {
    window: Window,
    context: GraphicsContext,
    panel: Panel,
    canvas: Canvas,
    background: Option<Pixmap>,
}

impl Frame {
    pub fn new(event_loop: &EventLoop<()>, command: &Command) -> Result<Self> {
        let window = WindowBuilder::new().with_title("askew").build(event_loop)?;
        let context =
            unsafe { GraphicsContext::new(&window, &window) }.expect("Platform is not supported");
        let size = window.inner_size();
        let pixmap = Pixmap::new(size.width, size.height).expect("Size should be valid");
        let background = if let Some(path) = &command.background_path {
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
        let panel = Panel::new(pixmap, window_rectangle);
        let mut rng = rand::thread_rng();
        let points_vec = (0..command.random_points)
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
        let canvas = match command.curve_type {
            CurveType::Polyline => Canvas::new(
                canvas_rectangle,
                vec![Curve::ControlPoints(ControlPointsCurve::Polyline(
                    Polyline::new(points),
                ))],
                command,
            ),
            CurveType::Interpolation => Canvas::new(
                canvas_rectangle,
                vec![Curve::ControlPoints(ControlPointsCurve::Interpolation(
                    Interpolation::new(points, command.samples, command.chebyshev_nodes),
                ))],
                command,
            ),
            CurveType::Bezier => Canvas::new(
                canvas_rectangle,
                vec![Curve::ControlPoints(ControlPointsCurve::Bezier(
                    Bezier::new(points, command.samples, BezierAlgorithm::ChudyWozny),
                ))],
                command,
            ),
            CurveType::RationalBezier => {
                let points = (0..command.random_points)
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
                    vec![Curve::ControlPoints(ControlPointsCurve::RationalBezier(
                        RationalBezier::new(
                            points,
                            command.samples,
                            RationalBezierAlgorithm::ChudyWozny,
                        ),
                    ))],
                    command,
                )
            }
            CurveType::Trochoid => Canvas::new(
                Rectangle::new(Point::new(-2.0, -2.0), Size::new(4.0, 4.0)),
                vec![Curve::Formula(FormulaCurve::Trochoid(Trochoid::new(
                    5000,
                    (10.0 * -std::f32::consts::PI, 10.0 * std::f32::consts::PI),
                    0.3,
                    0.8,
                    0.3,
                    0.7,
                )))],
                command,
            ),
        };
        Ok(Self {
            window,
            context,
            panel,
            canvas,
            background,
        })
    }

    fn size_rectangle(size: PhysicalSize<u32>) -> Rectangle<u32> {
        let origin = Point::new(0, 0);
        let size = size.into();
        Rectangle::new(origin, size)
    }

    pub fn handle_event(&mut self, event: Option<CanvasEvent>) -> Result<()> {
        if let Some(event) = event {
            self.canvas.handle_event(event)?;
            self.window.request_redraw();
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let mut panel = self.panel.as_sub_panel();
        panel.fill(PaintColor::from_rgba(32, 32, 32, 255));
        if let Some(background) = &self.background {
            panel.draw_pixmap(0, 0, background.as_ref());
        }
        let size = self.panel.area().size();
        let split_layout = [size.height() as usize - 44, 22, 22];
        let [panel, status, command] = self.panel.split_vertical(split_layout);
        let _status_bar = Bar::new(status, "status")?;
        let _command_bar = Bar::new(command, ":command")?;
        self.canvas.rasterize(panel)?;
        let buffer = self.panel.buffer();
        let buffer = buffer.data();
        let buffer = bytemuck::cast_slice(buffer);
        self.context
            .set_buffer(buffer, size.width() as u16, size.height() as u16);
        Ok(())
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let pixmap = Pixmap::new(size.width, size.height).expect("Size should be valid");
        let rectangle = Self::size_rectangle(size);
        let panel = Panel::new(pixmap, rectangle);
        self.panel = panel
    }

    pub fn save(&mut self, format: SaveFormat) -> Result<()> {
        match format {
            SaveFormat::Png => {
                let panel = self.panel.as_sub_panel();
                self.canvas.rasterize(panel)?;
                let buffer = self.panel.buffer();
                let buffer = buffer.data();
                let buffer: &[[u8; 4]] = bytemuck::cast_slice(buffer);
                let buffer = buffer
                    .iter()
                    .copied()
                    .flat_map(|[b, g, r, _a]| [r, g, b])
                    .collect::<Vec<_>>();
                let size = self.panel.area().size();
                let image = RgbImage::from_raw(size.width(), size.height(), buffer)
                    .ok_or_else(|| anyhow!("Image should fit"))?;
                image.save_with_format("curve.png", ImageFormat::Png)?;
            }
        }
        Ok(())
    }

    pub fn has_id(&self, id: WindowId) -> bool {
        self.window.id() == id
    }
}
