use crate::canvas::curve::CurvePoint;

pub struct Interpolation {
    points: Vec<CurvePoint>,
    samples: u32,
    range: (f32, f32),
}

impl Interpolation {
    pub fn new(points: Vec<CurvePoint>, samples: u32, range: (f32, f32)) -> Self {
        Self {
            points,
            samples,
            range,
        }
    }

    pub fn line_approx_points(&self) -> impl Iterator<Item = CurvePoint> + '_ {
        let delta = self.range.1 - self.range.0;
        let times = (0..=self.samples)
            .map(move |index| self.range.0 + (index as f32 * delta) / self.samples as f32);
        let ts = (0..self.points.len())
            .map(move |index| self.range.0 + (index as f32 * delta) / self.points.len() as f32)
            .collect::<Vec<_>>();
        let (xs, ys): (Vec<_>, Vec<_>) = self.points.iter().map(|point| (*point).into()).unzip();
        // times
        //     .into_iter()
        //     .map(move |t| (t, self.lagrange(t, &xs, &ys)).into())
        times
            .into_iter()
            .map(move |t| (self.lagrange(t, &ts, &xs), self.lagrange(t, &ts, &ys)).into())
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
