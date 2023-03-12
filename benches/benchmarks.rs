use std::time::Duration;

use broute::graphs::algorithms::{
    form_abstracted_graph, travelling_salesman, ConnectedComponents, Dijkstra,
};
use broute::graphs::datastructures::{Digraph, NodeIndex};
use broute::graphs::input::{get_random_graph, load_pbf_file, load_tsplib_file, load_xgmml_file};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::seq::IteratorRandom;
use rand::Rng;

fn shortest_path_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Shortest path");

    let random_g = get_random_graph(3000, 0.5, 4.0, 1.0);
    group.bench_with_input(
        BenchmarkId::new("Random graph", &random_g),
        &random_g,
        |b, g| {
            b.iter(|| {
                let mut dj = Dijkstra::new(g, NodeIndex(0));
                dj.run();
            })
        },
    );

    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX);
    group.bench_with_input(
        BenchmarkId::new("DIMCAS d1291", &dimacs_g),
        &dimacs_g,
        |b, g| {
            b.iter(|| {
                let mut dj = Dijkstra::new(g, NodeIndex(0));
                dj.run();
            })
        },
    );

    let monaco_g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");
    let mut monaco_cc = ConnectedComponents::new(&monaco_g);
    monaco_cc.run();
    let monaco_largest_g = monaco_cc.get_largest_connected_subgraphs();
    group.bench_with_input(
        BenchmarkId::new("OSM Monaco", &monaco_largest_g),
        &monaco_largest_g,
        |b, g| {
            b.iter(|| {
                let mut dj = Dijkstra::new(g, NodeIndex(0));
                dj.run();
            })
        },
    );

    let dimacs_g = load_xgmml_file("test_data/dimacs_shortest_path/USA-road-d.NY.gr").unwrap();
    let mut rng = rand::thread_rng();
    group.bench_with_input(
        BenchmarkId::new("DIMACS USA-road-d.NY", &dimacs_g),
        &dimacs_g,
        |b, g| {
            b.iter(|| {
                let mut dj = Dijkstra::new(
                    g,
                    NodeIndex(rng.gen_range(1..=dimacs_g.num_vertices())),
                );
                dj.run();
                let p =
                    dj.get_graph_path(NodeIndex(rng.gen_range(1..=dimacs_g.num_vertices())));
                p.get_length_on_graph(&dimacs_g);
            })
        },
    );

    group.finish();
}

fn travelling_salesman_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Travelling salesman");

    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX);
    group.bench_with_input(
        BenchmarkId::new("DIMCAS d1291", &dimacs_g),
        &dimacs_g,
        |b, g| b.iter(|| travelling_salesman(g, false)),
    );

    let monaco_g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

    let mut monaco_cc = ConnectedComponents::new(&monaco_g);
    monaco_cc.run();
    let monaco_largest_g = monaco_cc.get_largest_connected_subgraphs();

    let all_node_ids = monaco_largest_g.nodes_data().get_node_ids();
    let selected_node_ids = all_node_ids
        .into_iter()
        .choose_multiple(&mut rand::thread_rng(), 5);

    let abstracted_graph = form_abstracted_graph(&monaco_largest_g, &selected_node_ids);
    group.bench_with_input(
        BenchmarkId::new("OSM Monaco - 5 random nodes", &abstracted_graph),
        &abstracted_graph,
        |b, g| b.iter(|| travelling_salesman(g, false)),
    );

    group.finish();
}

fn connected_components_benchmark(c: &mut Criterion) {
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

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
    targets = shortest_path_benchmark, travelling_salesman_benchmark, connected_components_benchmark
}
criterion_main!(benches);
