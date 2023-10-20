use tiny_skia::PixmapMut;

use crate::canvas::curve::bezier::BezierCurve;
use crate::canvas::curve::interpolation::InterpolationCurve;
use crate::canvas::curve::polyline::PolylineCurve;
use crate::canvas::curve::rational_bezier::RationalBezierCurve;
use crate::canvas::curve::trochoid::TrochoidCurve;
use crate::config::CurveType;

pub mod bezier;
pub mod interpolation;
pub mod polyline;
pub mod rational_bezier;
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
pub enum Curve {
    Polyline(Box<PolylineCurve>),
    Interpolation(Box<InterpolationCurve>),
    Bezier(Box<BezierCurve>),
    RationalBezier(Box<RationalBezierCurve>),
    Trochoid(Box<TrochoidCurve>),
}

impl Curve {
    #[must_use]
    pub fn curve_type(&self) -> CurveType {
        match self {
            Curve::Polyline(_) => CurveType::Polyline,
            Curve::Interpolation(_) => CurveType::Interpolation,
            Curve::Bezier(_) => CurveType::Bezier,
            Curve::RationalBezier(_) => CurveType::RationalBezier,
            Curve::Trochoid(_) => CurveType::Trochoid,
        }
    }
}

impl Update for Curve {
    fn update(&mut self) {
        match self {
            Curve::Polyline(curve) => curve.update(),
            Curve::Interpolation(curve) => curve.update(),
            Curve::Bezier(curve) => curve.update(),
            Curve::RationalBezier(curve) => curve.update(),
            Curve::Trochoid(curve) => curve.update(),
        }
    }
}

impl DrawOn for Curve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        match self {
            Curve::Polyline(curve) => curve.draw_on(pixmap),
            Curve::Interpolation(curve) => curve.draw_on(pixmap),
            Curve::Bezier(curve) => curve.draw_on(pixmap),
            Curve::RationalBezier(curve) => curve.draw_on(pixmap),
            Curve::Trochoid(curve) => curve.draw_on(pixmap),
        }
    }
}

impl Default for Curve {
    fn default() -> Self {
        Self::Polyline(Box::default())
    }
}
