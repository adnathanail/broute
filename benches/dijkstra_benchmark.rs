use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use broute::graphs;

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = graphs::digraph::Digraph::new(8);

    g.add_edge(0, 1, 5.0);
    g.add_edge(0, 4, 9.0);
    g.add_edge(0, 7, 8.0);
    g.add_edge(1, 2, 12.0);
    g.add_edge(1, 3, 15.0);
    g.add_edge(1, 7, 4.0);
    g.add_edge(2, 3, 3.0);
    g.add_edge(2, 6, 11.0);
    g.add_edge(3, 6, 9.0);
    g.add_edge(4, 5, 4.0);
    g.add_edge(4, 6, 20.0);
    g.add_edge(4, 7, 5.0);
    g.add_edge(5, 2, 1.0);
    g.add_edge(5, 6, 13.0);
    g.add_edge(7, 2, 7.0);
    g.add_edge(7, 5, 6.0);

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
