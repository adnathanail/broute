use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use broute::graphs;
use broute::graphs::algorithms::connected_components::ConnectedComponents;
use broute::graphs::algorithms::dijkstra::dijkstra;
use broute::graphs::algorithms::travelling_salesman::travelling_salesman;
use broute::graphs::datastructures::digraph::NodeIndex;

fn dijkstra_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dijkstra");

    let random_g = graphs::input::random_graph::get_random_graph(3000, 0.5, 4.0, 1.0);
    group.bench_with_input(
        BenchmarkId::new("Random graph", &random_g),
        &random_g,
        |b, random_g| b.iter(|| dijkstra(random_g, NodeIndex(0))),
    );

    let dimacs_g =
        graphs::input::tsplib::load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX);
    group.bench_with_input(
        BenchmarkId::new("DIMCAS d1291", &dimacs_g),
        &dimacs_g,
        |b, dimacs_g| b.iter(|| dijkstra(dimacs_g, NodeIndex(0))),
    );

    let monaco_g = graphs::input::pbf::load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");
    let mut monaco_cc =
        graphs::algorithms::connected_components::ConnectedComponents::new(&monaco_g);
    monaco_cc.run();
    let monaco_largest_g = monaco_cc.get_largest_connected_subgraphs();
    group.bench_with_input(
        BenchmarkId::new("OSM Monaco", &monaco_largest_g),
        &monaco_largest_g,
        |b, monaco_largest_g| b.iter(|| dijkstra(monaco_largest_g, NodeIndex(0))),
    );

    group.finish();
}

fn travelling_salesman_benchmark(c: &mut Criterion) {
    let g = graphs::input::tsplib::load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX);

    let mut group = c.benchmark_group("Travelling salesman");

    group.bench_with_input(BenchmarkId::new("DIMCAS d1291", &g), &g, |b, g| {
        b.iter(|| travelling_salesman(g, false))
    });

    group.finish();
}

fn connected_components_benchmark(c: &mut Criterion) {
    let g = graphs::input::pbf::load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

    let mut group = c.benchmark_group("Connected components");

    group.bench_with_input(BenchmarkId::new("OSM Monaco", &g), &g, |b, g| {
        b.iter(|| {
            let mut cc = ConnectedComponents::new(g);
            cc.run();
            cc.get_largest_connected_subgraphs()
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = dijkstra_benchmark, travelling_salesman_benchmark, connected_components_benchmark
}
criterion_main!(benches);
