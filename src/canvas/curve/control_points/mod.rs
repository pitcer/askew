use kind::convex_hull::ConvexHull;
use kind::interpolation::Interpolation;
use kind::polyline::Polyline;
use points::ControlPoints;

use crate::canvas::curve::control_points::event_handler::ControlPointsCurveEventHandler;
use crate::canvas::curve::control_points::kind::bezier::Bezier;
use crate::canvas::curve::control_points::kind::rational_bezier::RationalBezier;
use crate::canvas::math::point::Point;

pub mod event_handler;
pub mod kind;
pub mod points;

pub type CurvePoints = ControlPoints<CurvePoint>;

pub type CurvePoint = Point<f32>;

pub trait GetControlPoints {
    type Point: AsRef<CurvePoint>;

    fn control_points(&self) -> &ControlPoints<Self::Point>;

    fn control_points_mut(&mut self) -> &mut ControlPoints<Self::Point>;
}

#[derive(Debug)]
pub enum ControlPointsCurveKind {
    Polyline(Polyline),
    ConvexHull(ConvexHull),
    Interpolation(Interpolation),
    Bezier(Bezier),
    RationalBezier(RationalBezier),
}

impl ControlPointsCurveKind {
    pub fn event_handler(&mut self) -> ControlPointsCurveEventHandler<'_> {
        ControlPointsCurveEventHandler::new(self)
    }
}

#[derive(Debug, Copy, Clone)]
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

    pub fn weight(self) -> W {
        self.weight
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
