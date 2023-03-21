use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::{form_abstracted_graph, ConnectedComponents, SimulatedAnnealing};
use broute::graphs::datastructures::{Digraph, NodeID, NodeIndex};
use broute::graphs::input::{load_pbf_file, load_tsplib_file};
use rand::seq::IteratorRandom;

#[test]
fn travelling_salesman_dimacs_test() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX);

    travelling_salesman(&dimacs_g, true);
}

fn check_graph_adjacency(
    g: &impl Digraph,
    node_id: &NodeID,
    expected_adjacency: Vec<(NodeIndex, f64)>,
) {
    let node_index = *g.nodes_data().get_node_index_by_id(node_id);
    let actual_adjacency: Vec<(NodeIndex, f64)> = g
        .adj(node_index)
        .into_iter()
        .map(|adjacency| (adjacency.node_index, adjacency.weight))
        .collect();
    assert_eq!(actual_adjacency, expected_adjacency);
}

#[test]
fn a_star_travelling_salesman_integration_test() {
    // Load graph
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf").unwrap();

    // Get largest connected subgraph
    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();
    assert_eq!(c_g.num_vertices(), 9936);

    // 5 points across Monaco
    let all_node_ids = c_g.nodes_data().get_node_ids();
    let selected_node_ids = all_node_ids
        .into_iter()
        .choose_multiple(&mut rand::thread_rng(), 10);

    // Form abstracted graph
    let abstracted_graph = form_abstracted_graph(&c_g, &selected_node_ids);

    // output_graph_to_file(
    //     &abstracted_graph,
    //     String::from("out/pickup_node_graph.svg"),
    // );

    // Run TSP
    let mut path_lengths: Vec<f64> = vec![];
    for _ in 0..100 {
        let mut sa = SimulatedAnnealing::new(&abstracted_graph);
        sa.run(100.0, 0.99, 100);
        path_lengths.push(sa.get_best_path().get_length_on_graph(&abstracted_graph));
    }

    let average_path_length = path_lengths.into_iter().sum::<f64>() / 100.0;
    println!("{average_path_length}");
    assert!(average_path_length < 8.1);

    // output_graph_to_file_with_path(
    //     &abstracted_graph,
    //     &best_path,
    //     String::from("out/pickup_node_graph_with_path.svg"),
    // );
}
