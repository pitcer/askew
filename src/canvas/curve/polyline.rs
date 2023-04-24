use nalgebra::Point2;

pub struct Polyline {
    points: Vec<Point2<f32>>,
}

impl Polyline {
    pub fn new(points: Vec<Point2<f32>>) -> Self {
        Self { points }
    }

    pub fn line_approx_points(&self) -> impl Iterator<Item = &Point2<f32>> {
        self.points.iter()
    }

    pub fn add_point(&mut self, point: Point2<f32>) {
        self.points.push(point)
    }
}
