use std::num::NonZeroU32;

use anyhow::{anyhow, Result};
use image::{EncodableLayout, ImageFormat, RgbImage};
use rand::Rng;
use softbuffer::{Context, Surface};
use tiny_skia::IntSize;
use tiny_skia::Pixmap;
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowId};

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
use crate::ui::bar::TextPanel;
use crate::ui::color::{Alpha, Rgb};
use crate::ui::font::{FontLayout, FontLoader, GlyphRasterizer};
use crate::ui::panel::Panel;
use crate::ui::pixel::Pixel;

pub struct Frame {
    window: Window,
    _context: Context,
    surface: Surface,
    canvas: Canvas,
    background: Option<Pixmap>,
    font_loader: FontLoader,
    glyph_rasterizer: GlyphRasterizer,
    status_layout: FontLayout,
    command_layout: FontLayout,
}

impl Frame {
    pub fn new(window: Window, command: &Command) -> Result<Self> {
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
        let font_loader = FontLoader::new(&command.font_path)?;
        let glyph_rasterizer = GlyphRasterizer::new();
        let status_layout = FontLayout::new(command.font_size);
        let command_layout = FontLayout::new(command.font_size);
        Ok(Self {
            window,
            _context: context,
            surface,
            canvas,
            background,
            font_loader,
            glyph_rasterizer,
            status_layout,
            command_layout,
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
                "{} {}/{}",
                name,
                self.canvas.properties().current_curve + 1,
                self.canvas.curves().len()
            ));
        let mut status_bar = TextPanel::new(status, Rgb::new(249, 250, 244), Rgb::new(48, 48, 48));
        status_bar.fill();
        status_bar.draw_layout(
            &self.font_loader,
            &self.status_layout,
            &mut self.glyph_rasterizer,
        );

        self.command_layout
            .setup(&self.font_loader)
            .append_text(":command");
        let mut command_bar =
            TextPanel::new(command, Rgb::new(249, 250, 244), Rgb::new(48, 48, 48));
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

    pub fn has_id(&self, id: WindowId) -> bool {
        self.window.id() == id
    }
}
