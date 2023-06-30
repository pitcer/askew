use crate::{
    canvas::curve::control_points::kind::interpolation::event_handler::InterpolationEventHandler,
    canvas::curve::control_points::points::ControlPoints,
    canvas::curve::control_points::{CurvePoint, CurvePoints, GetControlPoints},
    canvas::curve::converter::{CurvePath, PathConverter, ToPath},
    canvas::curve::samples::Samples,
    canvas::math,
    canvas::math::point::Point,
};

pub mod event_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Interpolation {
    points: CurvePoints,
    samples: Samples,
    nodes: InterpolationNodes,
}

impl Interpolation {
    #[must_use]
    pub fn new(points: CurvePoints, samples: Samples, nodes: InterpolationNodes) -> Self {
        Self {
            points,
            samples,
            nodes,
        }
    }

    pub fn event_handler(&mut self) -> InterpolationEventHandler<'_> {
        InterpolationEventHandler::new(self)
    }
}

impl ToPath for Interpolation {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        let length = self.points.length();
        if length < 2 {
            return None;
        }

        let (ts, first, last) = match self.nodes {
            InterpolationNodes::Chebyshev => {
                let ts = (1..=length)
                    .map(|index| math::chebyshev(length, index))
                    .collect::<Vec<_>>();
                let first = ts[0];
                let last = ts[length - 1];
                (ts, first, last)
            }
            InterpolationNodes::EquallySpaced => {
                let ts = (0..length)
                    .map(|index| index as f32 / (length - 1) as f32)
                    .collect::<Vec<_>>();
                (ts, 0.0, 1.0)
            }
        };

        let (xs, ys): (Vec<_>, Vec<_>) =
            self.points.iterator().map(|point| (*point).into()).unzip();
        let path = self
            .samples
            .equally_spaced(first..=last)
            .map(move |t| Point::new(math::lagrange(t, &ts, &xs), math::lagrange(t, &ts, &ys)));
        let path = CurvePath::new_open(path);
        converter.to_path(path)
    }
}

impl GetControlPoints for Interpolation {
    type Point = CurvePoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }

    fn into_control_points(self) -> ControlPoints<Self::Point> {
        self.points
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum InterpolationNodes {
    EquallySpaced,
    Chebyshev,
}
