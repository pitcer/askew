use std::fs::File;
use std::path::Path;

use anyhow::Result;
use rand::Rng;
use tiny_skia::PixmapMut;

use control_points::point::{CurveControlPoints, WeightedPoint};
use shape::{DrawOn, Shape, Update};

use crate::canvas::base_line::VisualBaseLine;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::request::declare::AddPoint;
use crate::canvas::samples::Samples;
use crate::canvas::shape::bezier::{BezierCurve, BezierCurveProperties};
use crate::canvas::shape::interpolation::{InterpolationCurve, InterpolationCurveProperties};
use crate::canvas::shape::polyline::PolylineCurve;
use crate::canvas::shape::rational_bezier::{
    RationalBezierCurve, RationalBezierCurveProperties, WeightedControlPoints,
};
use crate::canvas::shape::trochoid::TrochoidCurve;
use crate::config::{CanvasConfig, ShapeType};
use crate::request::RequestHandlerMut;

pub mod base_line;
pub mod control_points;
pub mod control_points_curve;
pub mod math;
pub mod paint;
pub mod polygon;
pub mod properties;
pub mod request;
pub mod samples;
pub mod shape;
pub mod visual_path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    curves: Vec<Shape>,
    size: Rectangle<f32>,
    #[serde(skip)]
    properties: CanvasProperties,
    pub config: CanvasConfig,
}

impl Canvas {
    #[must_use]
    pub fn new(size: Rectangle<f32>, config: CanvasConfig) -> Self {
        let properties = CanvasProperties::default();
        let curve = Self::create_curve(&config, config.default_curve_type, None, None);
        let curves = vec![curve];
        Self { curves, size, properties, config }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Canvas> {
        let file = File::open(path)?;
        let canvas = serde_json::from_reader::<_, Canvas>(file)?;
        Ok(canvas)
    }

    #[must_use]
    pub fn create_curve(
        config: &CanvasConfig,
        curve_type: ShapeType,
        points: Option<Vec<Point<f32>>>,
        samples: Option<u32>,
    ) -> Shape {
        let points = points.unwrap_or_default();
        let samples = Samples::new(samples.unwrap_or(config.curve_samples) as usize);
        let mut curve = match curve_type {
            ShapeType::Polyline => Shape::Polyline(Box::new(PolylineCurve::new(
                CurveControlPoints::new(points),
                VisualControlPoints::from_config(config),
                VisualBaseLine::from_config(config),
            ))),
            ShapeType::Interpolation => Shape::Interpolation(Box::new(InterpolationCurve::new(
                CurveControlPoints::new(points),
                VisualControlPoints::from_config(config),
                VisualBaseLine::from_config(config),
                InterpolationCurveProperties::new(config.default_interpolation_nodes),
                samples,
            ))),
            ShapeType::Bezier => Shape::Bezier(Box::new(BezierCurve::new(
                CurveControlPoints::new(points),
                VisualControlPoints::from_config(config),
                VisualBaseLine::from_config(config),
                BezierCurveProperties::new(config.default_bezier_algorithm),
                samples,
            ))),
            ShapeType::RationalBezier => {
                let points = points
                    .into_iter()
                    .map(|point| WeightedPoint::new(point, config.default_rational_bezier_weight))
                    .collect();
                Shape::RationalBezier(Box::new(RationalBezierCurve::new(
                    WeightedControlPoints::new(points),
                    VisualControlPoints::from_config(config),
                    VisualBaseLine::from_config(config),
                    RationalBezierCurveProperties::new(config.default_rational_bezier_algorithm),
                    samples,
                )))
            }
            ShapeType::Trochoid => Shape::Trochoid(Box::new(TrochoidCurve::new(
                VisualBaseLine::from_config(config),
                config.default_trochoid_properties,
                samples,
            ))),
            // TODO:
            ShapeType::RegularPolygon => Shape::RegularPolygon(Box::default()),
        };
        curve.update();
        curve
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn resize(&mut self, size: Rectangle<f32>) {
        self.size = size;
    }

    pub fn draw_on_all(&self, pixmap: &mut PixmapMut<'_>) {
        for curve in &self.curves {
            curve.draw_on(pixmap);
        }
    }

    #[deprecated(note = "Remove after implementing updates in event handler")]
    pub fn update_all(&mut self) {
        for curve in &mut self.curves {
            curve.update();
        }
    }

    pub fn generate_random_points(&mut self, number_of_points: u32) -> Result<()> {
        let mut random = rand::thread_rng();
        let origin = self.size.origin();
        let size = self.size.size();

        for _ in 0..number_of_points {
            let horizontal = random.gen_range(origin.horizontal()..=size.width());
            let vertical = random.gen_range(origin.vertical()..=size.height());
            let point = Point::new(horizontal, vertical);
            self.handle_mut(AddPoint::new(point))?;
        }
        Ok(())
    }

    #[must_use]
    pub fn curve_type(&self) -> ShapeType {
        self.current_curve().curve_type()
    }

    #[must_use]
    pub fn current_curve(&self) -> &Shape {
        &self.curves[self.properties.current_curve]
    }

    pub fn current_curve_mut(&mut self) -> &mut Shape {
        &mut self.curves[self.properties.current_curve]
    }

    #[must_use]
    pub fn curves(&self) -> &Vec<Shape> {
        &self.curves
    }

    #[must_use]
    pub fn properties(&self) -> &CanvasProperties {
        &self.properties
    }

    #[must_use]
    pub fn properties_mut(&mut self) -> &mut CanvasProperties {
        &mut self.properties
    }

    #[must_use]
    pub fn config(&self) -> &CanvasConfig {
        &self.config
    }

    #[must_use]
    pub fn size(&self) -> Rectangle<f32> {
        self.size
    }
}
