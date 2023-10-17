use tiny_skia::PixmapMut;

use crate::canvas::curve::event_handler::CurveEventHandler;
use crate::canvas::v2::curve::bezier::BezierCurve;
use crate::canvas::v2::curve::interpolation::InterpolationCurve;
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::canvas::v2::curve::rational_bezier::RationalBezierCurve;
use crate::canvas::v2::curve::trochoid::TrochoidCurve;
use crate::canvas::v2::{DrawOn, Update};
use crate::config::CurveType;

pub mod control_points;
pub mod event_handler;
pub mod formula;
pub mod samples;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Curve {
    Polyline(Box<PolylineCurve>),
    Interpolation(Box<InterpolationCurve>),
    Bezier(Box<BezierCurve>),
    RationalBezier(Box<RationalBezierCurve>),
    Trochoid(Box<TrochoidCurve>),
}

impl Curve {
    pub fn event_handler(&self) -> CurveEventHandler<'_> {
        CurveEventHandler::new(self)
    }

    pub fn event_handler_mut(&mut self) -> CurveEventHandler<'_> {
        CurveEventHandler::new(self)
    }

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
