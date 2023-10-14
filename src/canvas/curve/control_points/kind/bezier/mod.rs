use crate::canvas::curve::control_points::kind::bezier::event_handler::BezierEventHandler;
use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::{CurvePoint, CurvePoints, GetControlPoints};
use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};
use crate::canvas::curve::samples::Samples;
use crate::canvas::math;

pub mod event_handler;

#[deprecated]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Bezier {
    points: CurvePoints,
    samples: Samples,
    algorithm: BezierCurveAlgorithm,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum BezierCurveAlgorithm {
    #[deprecated(note = "This fails for n > 13")]
    Generic,
    #[default]
    DeCasteljau,
    ChudyWozny,
}

impl Bezier {
    #[must_use]
    pub fn new(points: CurvePoints, samples: Samples, algorithm: BezierCurveAlgorithm) -> Self {
        Self { points, samples, algorithm }
    }

    pub fn event_handler(&mut self) -> BezierEventHandler<'_> {
        BezierEventHandler::new(self)
    }

    fn bezier(&self, t: f32) -> CurvePoint {
        let n = self.points.length() as u32 - 1;
        self.points
            .iterator()
            .enumerate()
            .map(|(k, point)| {
                let bernstein = math::bernstein(n, k as u32, t);
                CurvePoint::new(point.horizontal() * bernstein, point.vertical() * bernstein)
            })
            .reduce(|accumulator, point| {
                CurvePoint::new(
                    accumulator.horizontal() + point.horizontal(),
                    accumulator.vertical() + point.vertical(),
                )
            })
            .expect("points should not be empty")
    }
}

impl ToPath for Bezier {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        if self.points.length() < 2 {
            return None;
        }

        let path = self.samples.equally_spaced(0.0..=1.0);
        match self.algorithm {
            BezierCurveAlgorithm::Generic => {
                let path = path.map(|t| self.bezier(t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
            BezierCurveAlgorithm::DeCasteljau => {
                let path = path.map(|t| math::de_casteljau(self.points.as_slice(), t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
            BezierCurveAlgorithm::ChudyWozny => {
                let path = path.map(|t| math::chudy_wozny(self.points.as_slice(), t));
                let path = CurvePath::new_open(path);
                converter.to_path(path)
            }
        }
    }
}

impl GetControlPoints for Bezier {
    type Point = CurvePoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }

    fn into_control_points(self) -> ControlPoints<Self::Point> {
        self.points
    }
}
