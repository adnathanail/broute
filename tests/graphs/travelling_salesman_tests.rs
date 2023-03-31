use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::{
    form_abstracted_graph, two_opt, two_opt_cost, ConnectedComponents, HillClimbing,
};
use broute::graphs::datastructures::{Digraph, GraphPath, NodeID, NodeIndex};
use broute::graphs::input::{load_pbf_file, load_tsplib_file};
use float_cmp::approx_eq;
use rand_distr::num_traits::abs;

#[test]
fn travelling_salesman_dimacs_test() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();

    let mut sa = HillClimbing::new(&dimacs_g);
    sa.run();
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

    for i in 0..actual_adjacency.len() {
        assert_eq!(actual_adjacency[i].0, expected_adjacency[i].0);
        assert!(approx_eq!(
            f64,
            actual_adjacency[i].1,
            expected_adjacency[i].1
        ));
    }
}

#[test]
fn a_star_travelling_salesman_integration_test() {
    // Load graph
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf").unwrap();

    // Get largest connected subgraph
    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();
    assert_eq!(c_g.num_vertices(), 15447);

    // 5 points across Monaco
    let pickup_points: Vec<LatLng> = Vec::from([
        LatLng {
            latitude: 43.7290416923665,
            longitude: 7.4111366271972665,
        },
        LatLng {
            latitude: 43.732592386855366,
            longitude: 7.416672706604005,
        },
        LatLng {
            latitude: 43.727630659255674,
            longitude: 7.419397830963136,
        },
        LatLng {
            latitude: 43.732297795757404,
            longitude: 7.425642013549806,
        },
        LatLng {
            latitude: 43.739507109017445,
            longitude: 7.42926836013794,
        },
    ]);

    // Get ID of closest node for each point
    let mut pickup_node_ids: Vec<NodeID> = vec![];
    for lat_lng in pickup_points {
        let node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(lat_lng);
        let node_id = c_g.nodes_data().get_node_id_by_index(&node_index);
        pickup_node_ids.push(*node_id);
    }
    assert_eq!(
        pickup_node_ids,
        [
            NodeID(6481411791),
            NodeID(1096589580),
            NodeID(1204303590),
            NodeID(1573112159),
            NodeID(1736929694),
        ]
    );

    // Form abstracted graph
    let abstracted_graph = form_abstracted_graph(&c_g, &pickup_node_ids);
    check_graph_adjacency(
        &abstracted_graph,
        &pickup_node_ids[0],
        vec![
            (NodeIndex(1), 0.840397472251362),
            (NodeIndex(2), 1.2553293767937947),
            (NodeIndex(3), 1.3997895536765643),
            (NodeIndex(4), 2.110387455814518),
        ],
    );
    check_graph_adjacency(
        &abstracted_graph,
        &pickup_node_ids[1],
        vec![
            (NodeIndex(0), 0.840397472251362),
            (NodeIndex(2), 0.8693174301431682),
            (NodeIndex(3), 0.8866103580347413),
            (NodeIndex(4), 1.4913841189321089),
        ],
    );
    check_graph_adjacency(
        &abstracted_graph,
        &pickup_node_ids[2],
        vec![
            (NodeIndex(0), 1.2553293767937947),
            (NodeIndex(1), 0.8693174301431683),
            (NodeIndex(3), 1.2416255386102428),
            (NodeIndex(4), 2.062933904435471),
        ],
    );
    check_graph_adjacency(
        &abstracted_graph,
        &pickup_node_ids[3],
        vec![
            (NodeIndex(0), 1.3997895536765645),
            (NodeIndex(1), 0.8866103580347408),
            (NodeIndex(2), 1.2416255386102426),
            (NodeIndex(4), 1.4367179661286307),
        ],
    );
    check_graph_adjacency(
        &abstracted_graph,
        &pickup_node_ids[4],
        vec![
            (NodeIndex(0), 2.1103874558145184),
            (NodeIndex(1), 1.4913841189321084),
            (NodeIndex(2), 2.0629339044354715),
            (NodeIndex(3), 1.4367179661286307),
        ],
    );

    // Run TSP
    let mut path_lengths: Vec<f64> = vec![];
    for _ in 0..100 {
        let mut sa = HillClimbing::new(&abstracted_graph);
        sa.run();
        path_lengths.push(sa.get_best_path().get_length_on_graph(&abstracted_graph));
    }

    assert!(path_lengths.iter().fold(f64::INFINITY, |a, &b| a.min(b)) < 8.0);
}

#[test]
fn two_opt_test() {
    let g = load_tsplib_file("test_data/dimacs_tsp/test.tsp", usize::MAX).unwrap();
    let path = GraphPath {
        path: vec![
            NodeIndex(0),
            NodeIndex(1),
            NodeIndex(2),
            NodeIndex(3),
            NodeIndex(4),
            NodeIndex(5),
            NodeIndex(6),
            NodeIndex(7),
            NodeIndex(8),
            NodeIndex(9),
        ],
    };
    for i in 0..10 {
        for j in 0..10 {
            if i != j {
                let new_path = two_opt(&path, i, j);
                let new_path_length = two_opt_cost(&g, &path, i, j);
                let actual_new_path_length =
                    new_path.get_length_on_graph(&g) - path.get_length_on_graph(&g);
                // Close enough
                assert!(abs(new_path_length - actual_new_path_length) < 0.0000000000002);
            }
        }
    }
}
