use crate::canvas::curve::CurvePoint;

#[derive(Debug)]
pub struct Interpolation {
    points: Vec<CurvePoint>,
    samples: u32,
    chebyshev_nodes: bool,
}

impl Interpolation {
    pub fn new(points: Vec<CurvePoint>, samples: u32, chebyshev_nodes: bool) -> Self {
        Self {
            points,
            samples,
            chebyshev_nodes,
        }
    }

    pub fn line_approx_points(&self) -> Option<impl Iterator<Item = CurvePoint> + '_> {
        if self.points.len() < 2 {
            return None;
        }

        let (ts, first, last) = if self.chebyshev_nodes {
            let ts = (1..=self.points.len())
                .map(|index| Self::chebyshev(self.points.len(), index))
                .collect::<Vec<_>>();
            let first = ts[0];
            let last = ts[self.points.len() - 1];
            (ts, first, last)
        } else {
            let ts = (0..self.points.len())
                .map(|index| index as f32 / (self.points.len() - 1) as f32)
                .collect::<Vec<_>>();
            (ts, 0.0, 1.0)
        };

        let times = (0..self.samples)
            .map(move |index| (index as f32 / (self.samples - 1) as f32) * (last - first) + first);
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

    fn chebyshev(n: usize, k: usize) -> f32 {
        f32::cos((2 * k - 1) as f32 * std::f32::consts::PI / (2 * n) as f32)
    }
}
