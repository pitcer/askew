use crate::canvas::curve::control_points::{
    ControlPoints, CurvePoint, CurvePoints, GetControlPoints,
};
use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};
use crate::canvas::{curve, math};
use crate::canvas::math::point::Point;

#[derive(Debug)]
pub struct Interpolation {
    points: CurvePoints,
    samples: u32,
    chebyshev_nodes: bool,
}

impl Interpolation {
    #[must_use]
    pub fn new(points: CurvePoints, samples: u32, chebyshev_nodes: bool) -> Self {
        Self {
            points,
            samples,
            chebyshev_nodes,
        }
    }
}

impl ToPath for Interpolation {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        let length = self.points.length();
        if length < 2 {
            return None;
        }

        let (ts, first, last) = if self.chebyshev_nodes {
            let ts = (1..=length)
                .map(|index| math::chebyshev(length, index))
                .collect::<Vec<_>>();
            let first = ts[0];
            let last = ts[length - 1];
            (ts, first, last)
        } else {
            let ts = (0..length)
                .map(|index| index as f32 / (length - 1) as f32)
                .collect::<Vec<_>>();
            (ts, 0.0, 1.0)
        };

        let (xs, ys): (Vec<_>, Vec<_>) =
            self.points.iterator().map(|point| (*point).into()).unzip();
        let path = curve::equally_spaced(first..=last, self.samples as usize)
            .map(move |t| Point::new(math::lagrange(t, &ts, &xs), math::lagrange(t, &ts, &ys)));
        let path = CurvePath::new_closed(path);
        converter.to_path(path)
    }
}

impl GetControlPoints for Interpolation {
    type Point = CurvePoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }

    fn control_points_mut(&mut self) -> &mut ControlPoints<Self::Point> {
        &mut self.points
    }
}
