use crate::canvas::curve::control_points::rational_bezier::RationalBezierPoint;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::math::point::Point;

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

pub fn de_casteljau(points: &[CurvePoint], t: f32) -> CurvePoint {
    let mut w = Vec::from(points);
    for k in 1..(points.len()) {
        for i in 0..(points.len() - k) {
            w[i] = Point::new(
                (1.0 - t) * w[i].horizontal() + t * w[i + 1].horizontal(),
                (1.0 - t) * w[i].vertical() + t * w[i + 1].vertical(),
            )
        }
    }
    w[0]
}

pub fn chudy_wozny(points: &[CurvePoint], t: f32) -> CurvePoint {
    let n = points.len();
    let mut h = 1.0;
    let mut u = 1.0 - t;
    let n_1 = n + 1;
    let mut points = points.iter().enumerate();
    let mut q = *points.next().unwrap().1;
    if t <= 0.5 {
        u = t / u;
        for (k, point) in points {
            h = h * u * (n_1 - k) as f32;
            h = h / (k as f32 + h);
            q = Point::new(
                (1.0 - h) * q.horizontal() + h * point.horizontal(),
                (1.0 - h) * q.vertical() + h * point.vertical(),
            );
        }
    } else {
        u = u / t;
        for (k, point) in points {
            h = h * (n_1 - k) as f32;
            h = h / (k as f32 * u + h);
            q = Point::new(
                (1.0 - h) * q.horizontal() + h * point.horizontal(),
                (1.0 - h) * q.vertical() + h * point.vertical(),
            );
        }
    }
    q
}

pub fn rational_chudy_wozny(points: &[RationalBezierPoint], t: f32) -> CurvePoint {
    let n = points.len();
    let mut h = 1.0;
    let mut u = 1.0 - t;
    let n_1 = n + 1;
    let mut q = points[0].point();
    if t <= 0.5 {
        u = t / u;
        for k in 1..points.len() {
            h = h * u * (n_1 - k) as f32 * points[k].weight();
            h = h / (k as f32 * points[k - 1].weight() + h);
            q = Point::new(
                (1.0 - h) * q.horizontal() + h * points[k].point().horizontal(),
                (1.0 - h) * q.vertical() + h * points[k].point().vertical(),
            );
        }
    } else {
        u = u / t;
        for k in 1..points.len() {
            h = h * (n_1 - k) as f32 * points[k].weight();
            h = h / (k as f32 * u * points[k - 1].weight() + h);
            q = Point::new(
                (1.0 - h) * q.horizontal() + h * points[k].point().horizontal(),
                (1.0 - h) * q.vertical() + h * points[k].point().vertical(),
            );
        }
    }
    q
}