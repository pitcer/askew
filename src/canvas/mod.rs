use anyhow::Result;
use rand::Rng;

use crate::canvas::curve::control_points::kind::bezier::Bezier;
use crate::canvas::curve::control_points::kind::convex_hull::ConvexHull;
use crate::canvas::curve::control_points::kind::interpolation::Interpolation;
use crate::canvas::curve::control_points::kind::polyline::Polyline;
use crate::canvas::curve::control_points::kind::rational_bezier::{
    RationalBezier, RationalBezierPoints,
};
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
use crate::config::{Config, CurveType};
use crate::event::canvas::AddPoint;
use crate::event::EventHandler;
use crate::ui::frame::panel::Panel;

pub mod curve;
pub mod event_handler;
pub mod math;
pub mod paint;
pub mod properties;
mod rasterizer;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    curves: Vec<CurveKind>,
    size: Rectangle<f32>,
    properties: CanvasProperties,
}

impl Canvas {
    #[must_use]
    pub fn new(size: Rectangle<f32>, config: &Config) -> Self {
        let properties = CanvasProperties::new(config);
        let curves = Vec::with_capacity(1);
        let mut canvas = Self {
            curves,
            size,
            properties,
        };
        let curve = canvas.create_curve(config.curve_type, None, None);
        canvas.curves.push(curve);
        canvas
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
    pub fn create_curve(
        &self,
        curve_type: CurveType,
        points: Option<Vec<Point<f32>>>,
        samples: Option<u32>,
    ) -> CurveKind {
        let points = points.unwrap_or_default();
        let samples = Samples::new(samples.unwrap_or(self.properties.samples) as usize);
        match curve_type {
            CurveType::Polyline => CurveKind::ControlPoints(ControlPointsCurveKind::Polyline(
                Polyline::new(CurvePoints::new(points)),
            )),
            CurveType::ConvexHull => CurveKind::ControlPoints(ControlPointsCurveKind::ConvexHull(
                ConvexHull::new(CurvePoints::new(points)),
            )),
            CurveType::Interpolation => {
                CurveKind::ControlPoints(ControlPointsCurveKind::Interpolation(Interpolation::new(
                    CurvePoints::new(points),
                    samples,
                    self.properties.interpolation_nodes,
                )))
            }
            CurveType::Bezier => {
                CurveKind::ControlPoints(ControlPointsCurveKind::Bezier(Bezier::new(
                    CurvePoints::new(points),
                    samples,
                    self.properties.bezier_algorithm,
                )))
            }
            CurveType::RationalBezier => {
                let points = points
                    .into_iter()
                    .map(|point| WeightedPoint::new(point, self.properties.default_weight))
                    .collect();
                CurveKind::ControlPoints(ControlPointsCurveKind::RationalBezier(
                    RationalBezier::new(
                        RationalBezierPoints::new(points),
                        samples,
                        self.properties.rational_bezier_algorithm,
                    ),
                ))
            }
            CurveType::Trochoid => CurveKind::Formula(FormulaCurveKind::Trochoid(Trochoid::new(
                samples,
                self.properties.trochoid_properties,
            ))),
        }
    }

    #[must_use]
    pub fn curve_type(&self) -> CurveType {
        match self.current_curve() {
            CurveKind::ControlPoints(curve) => match curve {
                ControlPointsCurveKind::Polyline(_) => CurveType::Polyline,
                ControlPointsCurveKind::ConvexHull(_) => CurveType::ConvexHull,
                ControlPointsCurveKind::Interpolation(_) => CurveType::Interpolation,
                ControlPointsCurveKind::Bezier(_) => CurveType::Bezier,
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
