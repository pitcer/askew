use std::fs::File;
use std::path::Path;

use anyhow::Result;
use rand::Rng;

use crate::canvas::curve::control_points::{ControlPointsCurveKind, CurvePoints, WeightedPoint};
use crate::canvas::curve::formula::trochoid::Trochoid;
use crate::canvas::curve::formula::FormulaCurveKind;
use crate::canvas::curve::samples::Samples;
use crate::canvas::curve::CurveKind;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::rasterizer::Rasterizer;
use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::curve::bezier::{BezierCurve, BezierCurveProperties};
use crate::canvas::v2::curve::interpolation::{InterpolationCurve, InterpolationCurveProperties};
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::canvas::v2::curve::rational_bezier::{
    RationalBezierCurve, RationalBezierCurveProperties, RationalBezierPoints,
};
use crate::canvas::v2::Update;
use crate::config::{CanvasConfig, CurveType};
use crate::event::canvas::AddPoint;
use crate::event::EventHandler;
use crate::ui::frame::panel::Panel;

pub mod curve;
pub mod event_handler;
pub mod math;
pub mod paint;
pub mod properties;
mod rasterizer;
pub mod v2;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    curves: Vec<CurveKind>,
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
    ) -> CurveKind {
        let points = points.unwrap_or_default();
        let samples = Samples::new(samples.unwrap_or(properties.samples) as usize);
        let curve = match curve_type {
            CurveType::Polyline => {
                let mut curve = PolylineCurve::new(
                    ControlPointsCurve::from_config(CurvePoints::new(points), config),
                    BasePolyline::from_config(config),
                );
                curve.update();
                CurveKind::ControlPoints(ControlPointsCurveKind::PolylineV2(Box::new(curve)))
            }
            CurveType::Interpolation => {
                let mut curve = InterpolationCurve::new(
                    ControlPointsCurve::from_config(CurvePoints::new(points), config),
                    BasePolyline::from_config(config),
                    InterpolationCurveProperties::new(properties.interpolation_nodes),
                    samples,
                );
                curve.update();
                CurveKind::ControlPoints(ControlPointsCurveKind::Interpolation(Box::new(curve)))
            }
            CurveType::BezierV2 => {
                let mut curve = BezierCurve::new(
                    ControlPointsCurve::from_config(CurvePoints::new(points), config),
                    BasePolyline::from_config(config),
                    BezierCurveProperties::new(properties.bezier_algorithm),
                    samples,
                );
                curve.update();
                CurveKind::ControlPoints(ControlPointsCurveKind::BezierV2(Box::new(curve)))
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
                CurveKind::ControlPoints(ControlPointsCurveKind::RationalBezier(Box::new(curve)))
            }
            CurveType::Trochoid => CurveKind::Formula(FormulaCurveKind::Trochoid(Trochoid::new(
                samples,
                properties.trochoid_properties,
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

    pub fn rasterize(&self, mut panel: Panel<'_>) -> Result<()> {
        for curve in &self.curves {
            Rasterizer.rasterize(curve, &self.properties, &mut panel)?;
        }
        Ok(())
    }

    #[deprecated(note = "Remove after implementing updates in event handler")]
    pub fn update(&mut self) {
        for curve in &mut self.curves {
            match curve {
                CurveKind::ControlPoints(curve) => match curve {
                    ControlPointsCurveKind::BezierV2(curve) => curve.update(),
                    ControlPointsCurveKind::PolylineV2(curve) => curve.update(),
                    ControlPointsCurveKind::RationalBezier(curve) => curve.update(),
                    ControlPointsCurveKind::Interpolation(curve) => curve.update(),
                    _ => {}
                },
                CurveKind::Formula(_) => {}
            }
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
        match self.current_curve() {
            CurveKind::ControlPoints(curve) => match curve {
                ControlPointsCurveKind::PolylineV2(_) => CurveType::Polyline,
                ControlPointsCurveKind::Interpolation(_) => CurveType::Interpolation,
                ControlPointsCurveKind::BezierV2(_) => CurveType::BezierV2,
                ControlPointsCurveKind::RationalBezier(_) => CurveType::RationalBezier,
            },
            CurveKind::Formula(curve) => match curve {
                FormulaCurveKind::Trochoid(_) => CurveType::Trochoid,
            },
        }
    }

    #[must_use]
    pub fn current_curve(&self) -> &CurveKind {
        &self.curves[self.properties.current_curve]
    }

    pub fn current_curve_mut(&mut self) -> &mut CurveKind {
        &mut self.curves[self.properties.current_curve]
    }

    #[must_use]
    pub fn curves(&self) -> &Vec<CurveKind> {
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
