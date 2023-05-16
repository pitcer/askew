use anyhow::{anyhow, Result};
use image::{EncodableLayout, ImageFormat, RgbImage};
use rand::Rng;
use softbuffer::GraphicsContext;
use tiny_skia::Pixmap;
use tiny_skia_path::IntSize;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder, WindowId};

use crate::canvas::curve::bezier::Bezier;
use crate::canvas::curve::convex_hull::ConvexHull;
use crate::canvas::curve::interpolation::Interpolation;
use crate::canvas::curve::polyline::Polyline;
use crate::canvas::curve::rational_bezier::{RationalBezier, RationalBezierPoint};
use crate::canvas::curve::trochoid::Trochoid;
use crate::canvas::curve::Curve;
use crate::canvas::geometry::point::Point;
use crate::canvas::geometry::rectangle::Rectangle;
use crate::canvas::geometry::size::Size;
use crate::canvas::layout::Layout;
use crate::canvas::paint::BgraColor;
use crate::canvas::Canvas;
use crate::canvas::event::CanvasEvent;
use crate::command::{Command, CurveType, SaveFormat};

pub struct Frame {
    window: Window,
    context: GraphicsContext,
    layout: Layout,
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
        let layout = Layout::new(pixmap, window_rectangle);
        let mut rng = rand::thread_rng();
        let points = (0..command.random_points)
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
        let canvas = match command.curve_type {
            CurveType::Polyline => Canvas::new(
                canvas_rectangle,
                Curve::Polyline(Polyline::new(points)),
                command,
            ),
            CurveType::Interpolation => Canvas::new(
                canvas_rectangle,
                Curve::Interpolation(Interpolation::new(
                    points,
                    command.samples,
                    command.chebyshev_nodes,
                )),
                command,
            ),
            CurveType::Bezier => Canvas::new(
                canvas_rectangle,
                Curve::Bezier(Bezier::new(points, command.samples)),
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
                Canvas::new(
                    canvas_rectangle,
                    Curve::RationalBezier(RationalBezier::new(points, command.samples)),
                    command,
                )
            }
            CurveType::ConvexHull => Canvas::new(
                canvas_rectangle,
                Curve::ConvexHull(ConvexHull::new(points)),
                command,
            ),
            CurveType::Trochoid => Canvas::new(
                Rectangle::new(Point::new(-2.0, -2.0), Size::new(4.0, 4.0)),
                Curve::Trochoid(Trochoid::new(
                    5000,
                    (10.0 * -std::f32::consts::PI, 10.0 * std::f32::consts::PI),
                    0.3,
                    0.8,
                    0.3,
                    0.7,
                )),
                command,
            ),
        };
        Ok(Self {
            window,
            context,
            layout,
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
        self.layout.fill(BgraColor::from_rgba(32, 32, 32, 255));
        if let Some(background) = &self.background {
            self.layout.draw_pixmap(0, 0, background.as_ref());
        }
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

    pub fn save(&mut self, format: SaveFormat) -> Result<()> {
        match format {
            SaveFormat::Png => {
                let panel = self.layout.panel();
                self.canvas.rasterize(panel)?;
                let buffer = self.layout.buffer();
                let buffer = buffer.data();
                let buffer: &[[u8; 4]] = bytemuck::cast_slice(buffer);
                let buffer = buffer
                    .iter()
                    .copied()
                    .flat_map(|[b, g, r, _a]| [r, g, b])
                    .collect::<Vec<_>>();
                let size = self.layout.area().size();
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

    fn scale_position(&self, position: PhysicalPosition<f64>) -> Point<f32> {
        Point::new(position.x as f32, position.y as f32)
    }
}
