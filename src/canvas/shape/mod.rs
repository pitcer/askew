use tiny_skia::PixmapMut;

use crate::canvas::shape::bezier::BezierCurve;
use crate::canvas::shape::interpolation::InterpolationCurve;
use crate::canvas::shape::polyline::PolylineCurve;
use crate::canvas::shape::rational_bezier::RationalBezierCurve;
use crate::canvas::shape::regular_polygon::RegularPolygon;
use crate::canvas::shape::trochoid::TrochoidCurve;
use crate::config::ShapeType;

pub mod bezier;
pub mod interpolation;
pub mod polyline;
pub mod rational_bezier;
pub mod regular_polygon;
pub mod request;
pub mod trochoid;

// TODO: will this trait be useful anywhere?
pub trait DrawOn {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>);
}

// TODO: in event handler add mut events that will call that method (all curves
// must implement this trait)
pub trait Update {
    fn update(&mut self);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Shape {
    Polyline(Box<PolylineCurve>),
    Interpolation(Box<InterpolationCurve>),
    Bezier(Box<BezierCurve>),
    RationalBezier(Box<RationalBezierCurve>),
    Trochoid(Box<TrochoidCurve>),
    RegularPolygon(Box<RegularPolygon>),
}

impl Shape {
    #[must_use]
    pub fn curve_type(&self) -> ShapeType {
        match self {
            Shape::Polyline(_) => ShapeType::Polyline,
            Shape::Interpolation(_) => ShapeType::Interpolation,
            Shape::Bezier(_) => ShapeType::Bezier,
            Shape::RationalBezier(_) => ShapeType::RationalBezier,
            Shape::Trochoid(_) => ShapeType::Trochoid,
            Shape::RegularPolygon(_) => ShapeType::RegularPolygon,
        }
    }
}

impl Update for Shape {
    fn update(&mut self) {
        match self {
            Shape::Polyline(curve) => curve.update(),
            Shape::Interpolation(curve) => curve.update(),
            Shape::Bezier(curve) => curve.update(),
            Shape::RationalBezier(curve) => curve.update(),
            Shape::Trochoid(curve) => curve.update(),
            Shape::RegularPolygon(shape) => shape.update(),
        }
    }
}

impl DrawOn for Shape {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        match self {
            Shape::Polyline(curve) => curve.draw_on(pixmap),
            Shape::Interpolation(curve) => curve.draw_on(pixmap),
            Shape::Bezier(curve) => curve.draw_on(pixmap),
            Shape::RationalBezier(curve) => curve.draw_on(pixmap),
            Shape::Trochoid(curve) => curve.draw_on(pixmap),
            Shape::RegularPolygon(shape) => shape.draw_on(pixmap),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::Polyline(Box::default())
    }
}
