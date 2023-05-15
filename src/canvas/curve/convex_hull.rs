use crate::canvas::curve::CurvePoint;
use crate::canvas::geometry::convex_hull::GrahamScan;

#[derive(Debug)]
pub struct ConvexHull {
    points: Vec<CurvePoint>,
}

impl ConvexHull {
    pub fn new(points: Vec<CurvePoint>) -> Self {
        Self { points }
    }

    pub fn line_approx_points(&self) -> Option<impl Iterator<Item = CurvePoint> + '_> {
        let points_copy = self.points.clone();

        if self.points.len() < 3 {
            // Cloning vector with less than 3 elements is cheap and somehow we need to
            // satisfy the type checker.
            return Some(points_copy.into_iter());
        }

        let mut graham_scan = GrahamScan::new(points_copy);
        let hull = graham_scan.convex_hull();
        Some(hull.into_iter())
    }

    pub fn add_point(&mut self, point: CurvePoint) {
        self.points.push(point);
    }

    pub fn points(&self) -> &[CurvePoint] {
        &self.points
    }
}
