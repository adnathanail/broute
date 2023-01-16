use std::{fs, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use broute::graphs;

fn dijkstra_benchmark(c: &mut Criterion) {
    let g = graphs::input::random_graph::get_random_graph(3000, 0.5, 4.0, 1.0);

    let mut group = c.benchmark_group("Dijkstra (random graph)");

    group.bench_with_input(BenchmarkId::new("v1", &g), &g, |b, g| {
        b.iter(|| {
            graphs::algorithms::dijkstra::dijkstra(g, graphs::datastructures::digraph::NodeIndex(0))
        })
    });
    group.bench_with_input(BenchmarkId::new("v2", &g), &g, |b, g| {
        b.iter(|| graphs::algorithms::dijkstra::dijkstra2(g))
    });

    group.finish();

    let tsp_string = fs::read_to_string("test_data/dimacs_tsp/d1291.tsp").unwrap();
    let g = graphs::input::tsplib::load_tsplib_file(tsp_string, usize::MAX);

    let mut group = c.benchmark_group("Dijkstra (DIMCAS d1291)");

    group.bench_with_input(BenchmarkId::new("v1", &g), &g, |b, g| {
        b.iter(|| {
            graphs::algorithms::dijkstra::dijkstra(g, graphs::datastructures::digraph::NodeIndex(0))
        })
    });
    group.bench_with_input(BenchmarkId::new("v2", &g), &g, |b, g| {
        b.iter(|| graphs::algorithms::dijkstra::dijkstra2(g))
    });

    group.finish();
}

fn travelling_salesman_benchmark(c: &mut Criterion) {
    let tsp_string = fs::read_to_string("test_data/dimacs_tsp/d1291.tsp").unwrap();
    let g = graphs::input::tsplib::load_tsplib_file(tsp_string, usize::MAX);

    let mut group = c.benchmark_group("Travelling salesman (DIMCAS d1291)");

    group.bench_with_input(BenchmarkId::new("v1", &g), &g, |b, g| {
        b.iter(|| graphs::algorithms::travelling_salesman::travelling_salesman(g, false))
    });

    group.finish();
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = dijkstra_benchmark, travelling_salesman_benchmark
}
criterion_main!(benches);
