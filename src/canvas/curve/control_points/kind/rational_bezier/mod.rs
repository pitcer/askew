use crate::canvas::v2::curve::rational_bezier::RationalBezierCurveAlgorithm;
use crate::{
    canvas::curve::control_points::kind::rational_bezier::event_handler::RationalBezierEventHandler,
    canvas::curve::control_points::points::ControlPoints,
    canvas::curve::control_points::{GetControlPoints, WeightedPoint},
    canvas::curve::converter::{CurvePath, PathConverter, ToPath},
    canvas::curve::samples::Samples,
    canvas::math,
};

pub mod event_handler;

#[deprecated]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RationalBezier {
    points: RationalBezierPoints,
    samples: Samples,
    algorithm: RationalBezierCurveAlgorithm,
}

impl RationalBezier {
    #[must_use]
    pub fn new(
        points: RationalBezierPoints,
        samples: Samples,
        algorithm: RationalBezierCurveAlgorithm,
    ) -> Self {
        Self { points, samples, algorithm }
    }

    pub fn event_handler(&mut self) -> RationalBezierEventHandler<'_> {
        RationalBezierEventHandler::new(self)
    }
}

pub type RationalBezierPoints = ControlPoints<RationalBezierPoint>;
pub type RationalBezierWeight = f32;
pub type RationalBezierPoint = WeightedPoint<f32, RationalBezierWeight>;

impl ToPath for RationalBezier {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        if self.points.length() < 2 {
            return None;
        }

        let path = self.samples.equally_spaced(0.0..=1.0);
        match self.algorithm {
            RationalBezierCurveAlgorithm::DeCasteljau => {
                let path = path.map(|t| math::rational_de_casteljau(self.points.as_slice(), t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
            RationalBezierCurveAlgorithm::ChudyWozny => {
                let path = path.map(|t| math::rational_chudy_wozny(self.points.as_slice(), t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
        }
    }
}

impl GetControlPoints for RationalBezier {
    type Point = RationalBezierPoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }

    fn into_control_points(self) -> ControlPoints<Self::Point> {
        self.points
    }
}
