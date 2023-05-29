pub mod convex_hull;
pub mod point;
pub mod rectangle;
pub mod size;
pub mod vector;

pub fn binomial_coefficient(n: u32, k: u32) -> u32 {
    ((n - k + 1)..=n).product::<u32>() / (1..=k).product::<u32>()
}

pub fn bernstein(n: u32, k: u32, t: f32) -> f32 {
    binomial_coefficient(n, k) as f32 * t.powi(k as i32) * (1.0 - t).powi((n - k) as i32)
}

pub fn lagrange(t: f32, xs: &[f32], ys: &[f32]) -> f32 {
    (0..xs.len()).map(|k| ys[k] * lambda(k, t, xs)).sum()
}

pub fn lambda(k: usize, t: f32, xs: &[f32]) -> f32 {
    (0..xs.len())
        .filter(|i| *i != k)
        .map(|i| (t - xs[i]) / (xs[k] - xs[i]))
        .product()
}

pub fn chebyshev(n: usize, k: usize) -> f32 {
    f32::cos((2 * k - 1) as f32 * std::f32::consts::PI / (2 * n) as f32)
}
