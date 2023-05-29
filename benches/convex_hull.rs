use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

use askew::canvas::math::convex_hull::GrahamScan;
use askew::canvas::math::point::Point;

fn bench_convex_hull(criterion: &mut Criterion) {
    let mut random = rand::thread_rng();
    let points = (0..100_000)
        .map(|_| Point::new(random.gen_range(0.0..=1.0), random.gen_range(0.0..=1.0)))
        .collect::<Vec<_>>();
    let graham_scan = GrahamScan::new(points);

    criterion.bench_function("GrahamScan::convex_hull", |bencher| {
        bencher.iter(|| graham_scan.clone().convex_hull())
    });
}

criterion_group!(benches, bench_convex_hull);
criterion_main!(benches);
