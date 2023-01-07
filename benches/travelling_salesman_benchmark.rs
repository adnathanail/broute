use std::{fs, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use broute::graphs;

fn criterion_benchmark(c: &mut Criterion) {
    let tsp_string = fs::read_to_string("test_data/dimacs_tsp/d1291.tsp").unwrap();
    let g = graphs::tsplib::load_tsplib_file(tsp_string, usize::max_value());

    let mut group = c.benchmark_group("Travelling salesman (DIMCAS d1291)");

    group.bench_with_input(BenchmarkId::new("v1", &g), &g, |b, g| {
        b.iter(|| graphs::travelling_salesman::travelling_salesman(g))
    });

    group.finish();
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = criterion_benchmark
}
criterion_main!(benches);
