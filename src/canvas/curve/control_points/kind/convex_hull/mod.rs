use crate::canvas::curve::control_points::kind::convex_hull::event_handler::ConvexHullEventHandler;
use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::{CurvePoint, CurvePoints, GetControlPoints};
use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};
use crate::canvas::math::convex_hull::GrahamScan;

pub mod event_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConvexHull {
    points: CurvePoints,
}

impl ConvexHull {
    #[must_use]
    pub fn new(points: CurvePoints) -> Self {
        Self { points }
    }

    pub fn event_handler(&mut self) -> ConvexHullEventHandler<'_> {
        ConvexHullEventHandler::new(self)
    }

    pub fn points_to_convex_hull_path<P>(
        points: Vec<CurvePoint>,
        converter: impl PathConverter<Path = P>,
    ) -> Option<P> {
        let graham_scan = GrahamScan::new(points);
        let hull = graham_scan.convex_hull();
        let path = CurvePath::new_closed(hull.into_iter());
        converter.to_path(path)
    }
}

impl ToPath for ConvexHull {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        let points = self.points.as_slice().to_vec();
        Self::points_to_convex_hull_path(points, converter)
    }
}

impl GetControlPoints for ConvexHull {
    type Point = CurvePoint;

    fn control_points(&self) -> &ControlPoints<Self::Point> {
        &self.points
    }
}
