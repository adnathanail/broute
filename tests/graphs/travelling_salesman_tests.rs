use broute::graphs::algorithms::{ConnectedComponents, form_abstracted_graph, travelling_salesman};
use broute::graphs::datastructures::{AMDigraph, Digraph, NodeID, NodeIndex};
use broute::graphs::input::load_pbf_file;

fn check_graph_adjacency(g: &dyn Digraph, node_id: &NodeID, expected_adjacency: Vec<(NodeIndex, f64)>) {
    let node_index = *g.nodes_data().get_node_index_by_id(node_id);
    let actual_adjacency: Vec<(NodeIndex, f64)> = g.adj(node_index).into_iter().map(|adjacency| (adjacency.node_index, adjacency.weight)).collect();
    assert_eq!(actual_adjacency, expected_adjacency);
}

#[test]
fn dijkstra_travelling_salesman_integration_test() {
    // Load graph
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

    // Get largest connected subgraph
    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();
    assert_eq!(c_g.num_vertices(), 9936);

    // 5 points across Monaco
    let pickup_points: Vec<[f64; 2]> = Vec::from([
        [43.7290416923665, 7.4111366271972665],
        [43.732592386855366, 7.416672706604005],
        [43.727630659255674, 7.419397830963136],
        [43.732297795757404, 7.425642013549806],
        [43.739507109017445, 7.42926836013794],
    ]);

    // Get ID of closest node for each point
    let mut pickup_node_ids: Vec<NodeID> = vec![];
    for point in pickup_points {
        let node_index = c_g
            .nodes_data()
            .get_node_index_closest_to_point(point[0], point[1]);
        let node_id = c_g.nodes_data().get_node_id_by_index(&node_index);
        pickup_node_ids.push(*node_id);
    }
    assert_eq!(pickup_node_ids, [NodeID(1074585047), NodeID(252362113), NodeID(1204303590), NodeID(1573112159), NodeID(1736929694)]);

    // Form abstracted graph
    let abstracted_graph = form_abstracted_graph(&c_g, &pickup_node_ids);
    check_graph_adjacency(&abstracted_graph, &pickup_node_ids[0], vec![(NodeIndex(0), 1.377158916250499), (NodeIndex(1), 0.9789951317313738), (NodeIndex(2), 1.755917633260777), (NodeIndex(3), 2.6659932095468)]);
    check_graph_adjacency(&abstracted_graph, &pickup_node_ids[1], vec![(NodeIndex(0), 1.980472813282169), (NodeIndex(1), 2.4817146592249784), (NodeIndex(2), 1.455229434276506), (NodeIndex(3), 2.0888624885817397)]);
    check_graph_adjacency(&abstracted_graph, &pickup_node_ids[2], vec![(NodeIndex(0), 2.075826825805118), (NodeIndex(1), 2.1511220126275212), (NodeIndex(2), 1.8695162433680703), (NodeIndex(3), 2.4192915795174095)]);
    check_graph_adjacency(&abstracted_graph, &pickup_node_ids[3], vec![(NodeIndex(0), 7.835010613747839), (NodeIndex(1), 7.604014959490413), (NodeIndex(2), 8.336252459690652), (NodeIndex(3), 6.42873712450868)]);
    check_graph_adjacency(&abstracted_graph, &pickup_node_ids[4], vec![(NodeIndex(0), 3.3510388777275084), (NodeIndex(1), 3.0842902626239828), (NodeIndex(2), 3.850361636228818), (NodeIndex(3), 2.921153828236195)]);

    // output_graph_to_file(
    //     &abstracted_graph,
    //     String::from("out/pickup_node_graph.svg"),
    // );

    // Run TSP
    let mut path_lengths: Vec<f64> = vec![];
    for _ in 0..100 {
        let best_path = travelling_salesman(&abstracted_graph, true);
        path_lengths.push(best_path.get_length_on_graph(&abstracted_graph));
    }
    assert_eq!((path_lengths.into_iter().sum::<f64>() / 100.0) < 8.0, true);

    // output_graph_to_file_with_path(
    //     &abstracted_graph,
    //     &best_path,
    //     String::from("out/pickup_node_graph_with_path.svg"),
    // );
}
