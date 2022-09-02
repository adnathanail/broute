use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use broute::graphs;

fn criterion_benchmark(c: &mut Criterion) {
    let g = graphs::random_graph::get_random_graph();

    let mut group = c.benchmark_group("Dijkstra");

    group.bench_with_input(BenchmarkId::new("v1", &g), &g, |b, g| {
        b.iter(|| graphs::dijkstra::dijkstra(g))
    });
    group.bench_with_input(BenchmarkId::new("v2", &g), &g, |b, g| {
        b.iter(|| graphs::dijkstra2::dijkstra2(g))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
