use crate::{
    canvas::curve::control_points::kind::rational_bezier::event_handler::RationalBezierEventHandler,
    canvas::curve::control_points::points::ControlPoints,
    canvas::curve::control_points::{CurvePoint, GetControlPoints, WeightedPoint},
    canvas::curve::converter::{CurvePath, PathConverter, ToPath},
    canvas::curve::samples::Samples,
    canvas::math,
};

pub mod event_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RationalBezier {
    points: RationalBezierPoints,
    samples: Samples,
    algorithm: RationalBezierAlgorithm,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum RationalBezierAlgorithm {
    Generic,
    #[default]
    DeCasteljau,
    ChudyWozny,
}

impl RationalBezier {
    #[must_use]
    pub fn new(
        points: RationalBezierPoints,
        samples: Samples,
        algorithm: RationalBezierAlgorithm,
    ) -> Self {
        Self { points, samples, algorithm }
    }

    pub fn event_handler(&mut self) -> RationalBezierEventHandler<'_> {
        RationalBezierEventHandler::new(self)
    }

    fn rational_bezier(&self, t: f32) -> CurvePoint {
        let n = self.points.length() as u32 - 1;
        let result = self
            .points
            .iterator()
            .enumerate()
            .map(|(k, point)| {
                let bernstein = math::bernstein(n, k as u32, t);
                CurvePoint::new(
                    point.point.horizontal() * bernstein * point.weight,
                    point.point.vertical() * bernstein * point.weight,
                )
            })
            .reduce(|accumulator, point| {
                CurvePoint::new(
                    accumulator.horizontal() + point.horizontal(),
                    accumulator.vertical() + point.vertical(),
                )
            })
            .expect("points should not be empty");
        let divisor = self
            .points
            .iterator()
            .enumerate()
            .map(|(k, point)| point.weight * math::bernstein(n, k as u32, t))
            .sum::<f32>();
        CurvePoint::new(result.horizontal() / divisor, result.vertical() / divisor)
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
            RationalBezierAlgorithm::Generic => {
                let path = path.map(|t| self.rational_bezier(t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
            RationalBezierAlgorithm::DeCasteljau => {
                let path = path.map(|t| math::rational_de_casteljau(self.points.as_slice(), t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
            RationalBezierAlgorithm::ChudyWozny => {
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
