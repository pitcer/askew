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
    values: ShapeCommonValues,
    default_values: &'a CanvasConfig,
}

#[derive(Debug, Default)]
pub struct ShapeCommonValues {
    pub points: Option<CurveControlPoints>,
    pub weighted_points: Option<WeightedControlPoints>,
    pub control_points: Option<VisualControlPoints>,
    pub open_base_line: Option<OpenBaseLine>,
    pub closed_base_line: Option<ClosedBaseLine>,
    pub samples: Option<Samples>,
    pub interpolation_properties: Option<InterpolationCurveProperties>,
    pub bezier_properties: Option<BezierCurveProperties>,
    pub rational_bezier_properties: Option<RationalBezierCurveProperties>,
    pub trochoid_properties: Option<TrochoidCurveProperties>,
}

impl<'a> ShapeChanger<'a> {
    #[must_use]
    pub fn new(default_values: &'a CanvasConfig) -> Self {
        let values = ShapeCommonValues::default();
        Self { values, default_values }
    }

    #[must_use]
    pub fn from_shape(shape: Shape, default_values: &'a CanvasConfig) -> Self {
        let values = shape.into();
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

    fn curve_control_points(&mut self) -> CurveControlPoints {
        // TODO: reuse weighted points if they are present
        self.values.points.take().unwrap_or_default()
    }

    fn weighted_control_points(&mut self) -> WeightedControlPoints {
        // TODO: reuse curve points if they are present
        self.values.weighted_points.take().unwrap_or_default()
    }

    fn control_points(&mut self) -> VisualControlPoints {
        take_or_from(&mut self.values.control_points, self.default_values)
    }

    fn open_base_line(&mut self) -> OpenBaseLine {
        take_or_from(&mut self.values.open_base_line, self.default_values)
    }

    fn closed_base_line(&mut self) -> ClosedBaseLine {
        take_or_from(&mut self.values.closed_base_line, self.default_values)
    }

    fn samples(&mut self) -> Samples {
        take_or_from(&mut self.values.samples, self.default_values)
    }

    fn interpolation_properties(&mut self) -> InterpolationCurveProperties {
        take_or_from(&mut self.values.interpolation_properties, self.default_values)
    }

    fn bezier_properties(&mut self) -> BezierCurveProperties {
        take_or_from(&mut self.values.bezier_properties, self.default_values)
    }

    fn rational_bezier_properties(&mut self) -> RationalBezierCurveProperties {
        take_or_from(&mut self.values.rational_bezier_properties, self.default_values)
    }

    fn trochoid_properties(&mut self) -> TrochoidCurveProperties {
        take_or_from(&mut self.values.trochoid_properties, self.default_values)
    }
}

fn take_or_from<T, U>(option: &mut Option<T>, default_value: U) -> T
where
    T: From<U>,
{
    option.take().unwrap_or_else(|| default_value.into())
}
