use tiny_skia::PixmapMut;

use points::ControlPoints;

use crate::canvas::curve::control_points::event_handler::ControlPointsCurveEventHandler;
use crate::canvas::math::point::Point;
use crate::canvas::v2::curve::bezier::BezierCurve;
use crate::canvas::v2::curve::interpolation::InterpolationCurve;
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::canvas::v2::curve::rational_bezier::RationalBezierCurve;
use crate::canvas::v2::{DrawOn, Update};

pub mod event_handler;
pub mod points;

pub type CurvePoints = ControlPoints<CurvePoint>;

pub type CurvePoint = Point<f32>;

pub trait GetControlPoints {
    type Point: AsRef<CurvePoint> + Into<CurvePoint> + Copy;

    fn control_points(&self) -> &ControlPoints<Self::Point>;

    fn into_control_points(self) -> ControlPoints<Self::Point>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ControlPointsCurveKind {
    PolylineV2(Box<PolylineCurve>),
    Interpolation(Box<InterpolationCurve>),
    BezierV2(Box<BezierCurve>),
    RationalBezier(Box<RationalBezierCurve>),
}

impl ControlPointsCurveKind {
    pub fn event_handler(&mut self) -> ControlPointsCurveEventHandler<'_> {
        ControlPointsCurveEventHandler::new(self)
    }
}

impl Update for ControlPointsCurveKind {
    fn update(&mut self) {
        match self {
            ControlPointsCurveKind::PolylineV2(curve) => curve.update(),
            ControlPointsCurveKind::Interpolation(curve) => curve.update(),
            ControlPointsCurveKind::BezierV2(curve) => curve.update(),
            ControlPointsCurveKind::RationalBezier(curve) => curve.update(),
        }
    }
}

impl DrawOn for ControlPointsCurveKind {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        match self {
            ControlPointsCurveKind::PolylineV2(curve) => curve.draw_on(pixmap),
            ControlPointsCurveKind::Interpolation(curve) => curve.draw_on(pixmap),
            ControlPointsCurveKind::BezierV2(curve) => curve.draw_on(pixmap),
            ControlPointsCurveKind::RationalBezier(curve) => curve.draw_on(pixmap),
        }
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct WeightedPoint<T, W> {
    point: Point<T>,
    weight: W,
}

impl<T, W> WeightedPoint<T, W> {
    pub fn new(point: Point<T>, weight: W) -> Self {
        Self { point, weight }
    }

    pub fn point(self) -> Point<T> {
        self.point
    }

    pub fn point_mut(&mut self) -> &mut Point<T> {
        &mut self.point
    }

    pub fn weight(self) -> W {
        self.weight
    }

    pub fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

impl<T, W> AsRef<Point<T>> for WeightedPoint<T, W> {
    fn as_ref(&self) -> &Point<T> {
        &self.point
    }
}

impl<T> AsRef<Point<T>> for Point<T> {
    fn as_ref(&self) -> &Point<T> {
        self
    }
}

impl<T, W> AsMut<Point<T>> for WeightedPoint<T, W> {
    fn as_mut(&mut self) -> &mut Point<T> {
        &mut self.point
    }
}

impl<T> AsMut<Point<T>> for Point<T> {
    fn as_mut(&mut self) -> &mut Point<T> {
        self
    }
}

impl<T, W> From<WeightedPoint<T, W>> for Point<T> {
    fn from(value: WeightedPoint<T, W>) -> Self {
        value.point
    }
}

impl<W> From<WeightedPoint<f32, W>> for tiny_skia::Point {
    fn from(value: WeightedPoint<f32, W>) -> Self {
        value.point.into()
    }
}
