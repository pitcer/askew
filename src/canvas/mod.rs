use std::fs::File;
use std::path::Path;

use anyhow::Result;
use rand::Rng;
use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::{CurvePoints, WeightedPoint};
use crate::canvas::curve::samples::Samples;
use crate::canvas::curve::Curve;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::curve::bezier::{BezierCurve, BezierCurveProperties};
use crate::canvas::v2::curve::interpolation::{InterpolationCurve, InterpolationCurveProperties};
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::canvas::v2::curve::rational_bezier::{
    RationalBezierCurve, RationalBezierCurveProperties, RationalBezierPoints,
};
use crate::canvas::v2::curve::trochoid::TrochoidCurve;
use crate::canvas::v2::{DrawOn, Update};
use crate::config::{CanvasConfig, CurveType};
use crate::event::canvas::AddPoint;
use crate::event::EventHandler;

pub mod curve;
pub mod event_handler;
pub mod math;
pub mod paint;
pub mod properties;
pub mod v2;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    curves: Vec<Curve>,
    size: Rectangle<f32>,
    #[serde(skip)]
    properties: CanvasProperties,
    config: CanvasConfig,
}

impl Canvas {
    #[must_use]
    pub fn new(size: Rectangle<f32>, config: CanvasConfig) -> Self {
        let properties = CanvasProperties::new(&config);
        let curve =
            Self::create_curve(&properties, &config, properties.default_curve_type, None, None);
        let curves = vec![curve];
        Self { curves, size, properties, config }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Canvas> {
        let file = File::open(path)?;
        let mut canvas = serde_json::from_reader::<_, Canvas>(file)?;
        canvas.properties = CanvasProperties::new(&canvas.config);
        Ok(canvas)
    }

    #[must_use]
    pub fn create_curve(
        properties: &CanvasProperties,
        config: &CanvasConfig,
        curve_type: CurveType,
        points: Option<Vec<Point<f32>>>,
        samples: Option<u32>,
    ) -> Curve {
        let points = points.unwrap_or_default();
        let samples = Samples::new(samples.unwrap_or(properties.samples) as usize);
        let curve = match curve_type {
            CurveType::Polyline => {
                let mut curve = PolylineCurve::new(
                    ControlPointsCurve::from_config(CurvePoints::new(points), config),
                    BasePolyline::from_config(config),
                );
                curve.update();
                Curve::Polyline(Box::new(curve))
            }
            CurveType::Interpolation => {
                let mut curve = InterpolationCurve::new(
                    ControlPointsCurve::from_config(CurvePoints::new(points), config),
                    BasePolyline::from_config(config),
                    InterpolationCurveProperties::new(properties.interpolation_nodes),
                    samples,
                );
                curve.update();
                Curve::Interpolation(Box::new(curve))
            }
            CurveType::Bezier => {
                let mut curve = BezierCurve::new(
                    ControlPointsCurve::from_config(CurvePoints::new(points), config),
                    BasePolyline::from_config(config),
                    BezierCurveProperties::new(properties.bezier_algorithm),
                    samples,
                );
                curve.update();
                Curve::Bezier(Box::new(curve))
            }
            CurveType::RationalBezier => {
                let points = points
                    .into_iter()
                    .map(|point| WeightedPoint::new(point, properties.default_weight))
                    .collect();
                let mut curve = RationalBezierCurve::new(
                    ControlPointsCurve::from_config(RationalBezierPoints::new(points), config),
                    BasePolyline::from_config(config),
                    RationalBezierCurveProperties::new(properties.rational_bezier_algorithm),
                    samples,
                );
                curve.update();
                Curve::RationalBezier(Box::new(curve))
            }
            CurveType::Trochoid => Curve::Trochoid(Box::new(TrochoidCurve::new(
                BasePolyline::from_config(config),
                properties.trochoid_properties,
                samples,
            ))),
        };
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

    pub fn event_handler(&mut self) -> CanvasEventHandler<'_> {
        CanvasEventHandler::new(self)
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
            self.event_handler().handle(AddPoint::new(point))?;
        }
        Ok(())
    }

    #[must_use]
    pub fn curve_type(&self) -> CurveType {
        self.current_curve().curve_type()
    }

    #[must_use]
    pub fn current_curve(&self) -> &Curve {
        &self.curves[self.properties.current_curve]
    }

    pub fn current_curve_mut(&mut self) -> &mut Curve {
        &mut self.curves[self.properties.current_curve]
    }

    #[must_use]
    pub fn curves(&self) -> &Vec<Curve> {
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
    pub fn size(&self) -> Rectangle<f32> {
        self.size
    }
}
