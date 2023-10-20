use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

use askew::canvas::curve::rational_bezier::RationalBezierPoint;
use askew::canvas::math;
use askew::canvas::math::point::Point;

fn bench_bezier(criterion: &mut Criterion) {
    let mut random = rand::thread_rng();
    let points = (0..10_000)
        .map(|_| Point::new(random.gen_range(0.0..=1.0), random.gen_range(0.0..=1.0)))
        .collect::<Vec<_>>();
    let t = criterion::black_box(0.42);

    let mut group = criterion.benchmark_group("bezier");
    group.bench_function("de_casteljau", |bencher| bencher.iter(|| math::de_casteljau(&points, t)));
    group.bench_function("chudy_wozny", |bencher| bencher.iter(|| math::chudy_wozny(&points, t)));

    let rational_points =
        points.iter().map(|point| RationalBezierPoint::new(*point, 1.0)).collect::<Vec<_>>();
    group.bench_function("rational_chudy_wozny_same_coefficients", |bencher| {
        bencher.iter(|| math::rational_chudy_wozny(&rational_points, t))
    });

    group.finish();
}

criterion_group!(benches, bench_bezier);
criterion_main!(benches);
