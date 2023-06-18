use anyhow::Result;

use crate::canvas::curve::control_points::kind::bezier::Bezier;
use crate::canvas::curve::control_points::kind::convex_hull::ConvexHull;
use crate::canvas::curve::control_points::kind::interpolation::Interpolation;
use crate::canvas::curve::control_points::kind::polyline::Polyline;
use crate::canvas::curve::control_points::kind::rational_bezier::{
    RationalBezier, RationalBezierPoints,
};
use crate::canvas::curve::control_points::{ControlPointsCurveKind, CurvePoints};
use crate::canvas::curve::formula::trochoid::Trochoid;
use crate::canvas::curve::formula::FormulaCurveKind;
use crate::canvas::curve::samples::Samples;
use crate::canvas::curve::CurveKind;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::rasterizer::Rasterizer;
use crate::config::{Config, CurveType};
use crate::ui::frame::panel::Panel;

pub mod curve;
pub mod event_handler;
pub mod math;
pub mod paint;
pub mod properties;
mod rasterizer;

#[derive(Debug)]
pub struct Canvas {
    curves: Vec<CurveKind>,
    properties: CanvasProperties,
}

impl Canvas {
    #[must_use]
    pub fn new(area: Rectangle<f32>, config: &Config) -> Self {
        let properties = CanvasProperties::new(area, config);
        let curves = Vec::with_capacity(1);
        let mut canvas = Self { curves, properties };
        let curve = canvas.create_curve(config.curve_type);
        canvas.curves.push(curve);
        canvas
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

    #[must_use]
    pub fn create_curve(&self, curve_type: CurveType) -> CurveKind {
        let points = Vec::new();
        let samples = Samples::new(self.properties.samples as usize);
        let curve_points = CurvePoints::new(points);
        match curve_type {
            CurveType::Polyline => CurveKind::ControlPoints(ControlPointsCurveKind::Polyline(
                Polyline::new(curve_points),
            )),
            CurveType::ConvexHull => CurveKind::ControlPoints(ControlPointsCurveKind::ConvexHull(
                ConvexHull::new(curve_points),
            )),
            CurveType::Interpolation => {
                CurveKind::ControlPoints(ControlPointsCurveKind::Interpolation(Interpolation::new(
                    curve_points,
                    samples,
                    self.properties.interpolation_nodes,
                )))
            }
            CurveType::Bezier => CurveKind::ControlPoints(ControlPointsCurveKind::Bezier(
                Bezier::new(curve_points, samples, self.properties.bezier_algorithm),
            )),
            CurveType::RationalBezier => CurveKind::ControlPoints(
                ControlPointsCurveKind::RationalBezier(RationalBezier::new(
                    RationalBezierPoints::new(vec![]),
                    samples,
                    self.properties.rational_bezier_algorithm,
                )),
            ),
            CurveType::Trochoid => CurveKind::Formula(FormulaCurveKind::Trochoid(Trochoid::new(
                samples,
                self.properties.trochoid_properties,
            ))),
        }
    }

    pub fn current_curve_mut(&mut self) -> &mut CurveKind {
        &mut self.curves[self.properties.current_curve]
    }

    #[must_use]
    pub fn curves(&self) -> &Vec<CurveKind> {
        &self.curves
    }

    #[must_use]
    pub fn properties_mut(&mut self) -> &mut CanvasProperties {
        &mut self.properties
    }

    #[must_use]
    pub fn properties(&self) -> &CanvasProperties {
        &self.properties
    }
}
