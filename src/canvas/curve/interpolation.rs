use crate::canvas::curve::CurvePoint;

pub struct Interpolation {
    points: Vec<CurvePoint>,
    samples: u32,
}

impl Interpolation {
    pub fn new(points: Vec<CurvePoint>, samples: u32) -> Self {
        Self { points, samples }
    }

    pub fn line_approx_points(&self) -> Option<impl Iterator<Item = CurvePoint> + '_> {
        if self.points.len() < 2 {
            return None;
        }

        let times = (0..self.samples).map(|index| index as f32 / (self.samples - 1) as f32);
        let ts = (0..self.points.len())
            .map(|index| index as f32 / (self.points.len() - 1) as f32)
            .collect::<Vec<_>>();

        let (xs, ys): (Vec<_>, Vec<_>) = self.points.iter().map(|point| (*point).into()).unzip();
        Some(times.map(move |t| (self.lagrange(t, &ts, &xs), self.lagrange(t, &ts, &ys)).into()))
    }

    pub fn add_point(&mut self, point: CurvePoint) {
        self.points.push(point)
    }

    pub fn points(&self) -> &[CurvePoint] {
        &self.points
    }

    fn lagrange(&self, t: f32, xs: &[f32], ys: &[f32]) -> f32 {
        (0..xs.len()).map(|k| ys[k] * self.lambda(k, t, xs)).sum()
    }

    fn lambda(&self, k: usize, t: f32, xs: &[f32]) -> f32 {
        (0..xs.len())
            .filter(|i| *i != k)
            .map(|i| (t - xs[i]) / (xs[k] - xs[i]))
            .product()
    }
}
