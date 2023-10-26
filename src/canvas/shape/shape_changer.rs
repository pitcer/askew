use crate::canvas::base_line::{ClosedBaseLine, OpenBaseLine};
use crate::canvas::control_points::point::CurveControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::samples::Samples;
use crate::canvas::shape::bezier::{BezierCurve, BezierCurveProperties};
use crate::canvas::shape::interpolation::{InterpolationCurve, InterpolationCurveProperties};
use crate::canvas::shape::polyline::PolylineCurve;
use crate::canvas::shape::rational_bezier::{
    RationalBezierCurve, RationalBezierCurveProperties, WeightedControlPoints,
};
use crate::canvas::shape::trochoid::{TrochoidCurve, TrochoidCurveProperties};
use crate::canvas::shape::{Shape, Update};
use crate::config::{CanvasConfig, ShapeType};

#[derive(Debug)]
pub struct ShapeChanger<'a> {
    values: CommonShapeValues,
    default_values: &'a CanvasConfig,
}

#[derive(Debug, Default)]
struct CommonShapeValues {
    points: Option<CurveControlPoints>,
    weighted_points: Option<WeightedControlPoints>,
    control_points: Option<VisualControlPoints>,
    open_base_line: Option<OpenBaseLine>,
    closed_base_line: Option<ClosedBaseLine>,
    samples: Option<Samples>,
    interpolation_properties: Option<InterpolationCurveProperties>,
    bezier_properties: Option<BezierCurveProperties>,
    rational_bezier_properties: Option<RationalBezierCurveProperties>,
    trochoid_properties: Option<TrochoidCurveProperties>,
}

impl<'a> ShapeChanger<'a> {
    #[must_use]
    pub fn new(default_values: &'a CanvasConfig) -> Self {
        let values = CommonShapeValues::default();
        Self { values, default_values }
    }

    #[must_use]
    pub fn from_shape(shape: Shape, default_values: &'a CanvasConfig) -> Self {
        let mut values = CommonShapeValues::default();
        match shape {
            Shape::Polyline(shape) => {
                values.points = Some(shape.points);
                values.control_points = Some(shape.control_points);
                values.open_base_line = Some(shape.base_line);
            }
            Shape::Interpolation(shape) => {
                values.points = Some(shape.points);
                values.control_points = Some(shape.control_points);
                values.open_base_line = Some(shape.polyline);
                values.samples = Some(shape.samples);
            }
            Shape::Bezier(shape) => {
                values.points = Some(shape.points);
                values.control_points = Some(shape.control_points);
                values.open_base_line = Some(shape.polyline);
                values.samples = Some(shape.samples);
            }
            Shape::RationalBezier(shape) => {
                values.weighted_points = Some(shape.points);
                values.control_points = Some(shape.control_points);
                values.open_base_line = Some(shape.base_line);
                values.samples = Some(shape.samples);
            }
            Shape::Trochoid(shape) => {
                values.open_base_line = Some(shape.base_line);
                values.samples = Some(shape.samples);
                values.trochoid_properties = Some(shape.properties);
            }
            Shape::RegularPolygon(_) => todo!(),
        }
        Self { values, default_values }
    }

    #[must_use]
    pub fn into_shape(mut self, shape_type: ShapeType) -> Shape {
        let mut shape = match shape_type {
            ShapeType::Polyline => Shape::Polyline(Box::new(PolylineCurve::new(
                self.curve_control_points(),
                self.control_points(),
                self.open_base_line(),
            ))),
            ShapeType::Interpolation => Shape::Interpolation(Box::new(InterpolationCurve::new(
                self.curve_control_points(),
                self.control_points(),
                self.open_base_line(),
                self.interpolation_properties(),
                self.samples(),
            ))),
            ShapeType::Bezier => Shape::Bezier(Box::new(BezierCurve::new(
                self.curve_control_points(),
                self.control_points(),
                self.open_base_line(),
                self.bezier_properties(),
                self.samples(),
            ))),
            ShapeType::RationalBezier => Shape::RationalBezier(Box::new(RationalBezierCurve::new(
                self.weighted_control_points(),
                self.control_points(),
                self.open_base_line(),
                self.rational_bezier_properties(),
                self.samples(),
            ))),
            ShapeType::Trochoid => Shape::Trochoid(Box::new(TrochoidCurve::new(
                self.open_base_line(),
                self.trochoid_properties(),
                self.samples(),
            ))),
            ShapeType::RegularPolygon => todo!(),
        };
        shape.update();
        shape
    }

    // TODO: instead of default use `default_values` to construct default properties
    fn curve_control_points(&mut self) -> CurveControlPoints {
        // TODO: reuse weighted points if they are present
        self.values.points.take().unwrap_or_default()
    }

    fn weighted_control_points(&mut self) -> WeightedControlPoints {
        // TODO: reuse curve points if they are present
        self.values.weighted_points.take().unwrap_or_default()
    }

    fn control_points(&mut self) -> VisualControlPoints {
        self.values.control_points.take().unwrap_or_default()
    }

    fn open_base_line(&mut self) -> OpenBaseLine {
        self.values.open_base_line.take().unwrap_or_default()
    }

    fn closed_base_line(&mut self) -> ClosedBaseLine {
        self.values.closed_base_line.take().unwrap_or_default()
    }

    fn samples(&mut self) -> Samples {
        self.values.samples.take().unwrap_or_default()
    }

    fn interpolation_properties(&mut self) -> InterpolationCurveProperties {
        self.values.interpolation_properties.take().unwrap_or_default()
    }

    fn bezier_properties(&mut self) -> BezierCurveProperties {
        self.values.bezier_properties.take().unwrap_or_default()
    }

    fn rational_bezier_properties(&mut self) -> RationalBezierCurveProperties {
        self.values.rational_bezier_properties.take().unwrap_or_default()
    }

    fn trochoid_properties(&mut self) -> TrochoidCurveProperties {
        self.values.trochoid_properties.take().unwrap_or_default()
    }
}
