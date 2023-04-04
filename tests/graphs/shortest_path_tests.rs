use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::{AStar, ConnectedComponents};
use broute::graphs::datastructures;
use broute::graphs::datastructures::{Digraph, NodeID, NodeIndex};
use broute::graphs::input::{load_pbf_file, load_xgmml_file};

#[test]
fn simple_a_star_test() {
    let mut g = datastructures::AMDigraph::new(8);

    for i in 0..8 {
        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(i),
            LatLng {
                latitude: 0.0,
                longitude: 0.0,
            },
        );
    }

    g.add_edge_by_id(NodeID(0), NodeID(1), 5.0);
    g.add_edge_by_id(NodeID(0), NodeID(4), 9.0);
    g.add_edge_by_id(NodeID(0), NodeID(7), 8.0);
    g.add_edge_by_id(NodeID(1), NodeID(2), 12.0);
    g.add_edge_by_id(NodeID(1), NodeID(3), 15.0);
    g.add_edge_by_id(NodeID(1), NodeID(7), 4.0);
    g.add_edge_by_id(NodeID(2), NodeID(3), 3.0);
    g.add_edge_by_id(NodeID(2), NodeID(6), 11.0);
    g.add_edge_by_id(NodeID(3), NodeID(6), 9.0);
    g.add_edge_by_id(NodeID(4), NodeID(5), 4.0);
    g.add_edge_by_id(NodeID(4), NodeID(6), 20.0);
    g.add_edge_by_id(NodeID(4), NodeID(7), 5.0);
    g.add_edge_by_id(NodeID(5), NodeID(2), 1.0);
    g.add_edge_by_id(NodeID(5), NodeID(6), 13.0);
    g.add_edge_by_id(NodeID(7), NodeID(2), 7.0);
    g.add_edge_by_id(NodeID(7), NodeID(5), 6.0);

    let mut astar1 = AStar::new(&g, NodeIndex(0), vec![NodeIndex(1)]);
    astar1.run();
    assert_eq!(astar1.get_dist_to_to_node(NodeIndex(1)), Some(5.0));
    // We didn't ask for it, so A* won't have prioritised it, so the answer will be bad
    assert_eq!(astar1.get_dist_to_to_node(NodeIndex(2)), None);

    let mut astar2 = AStar::new(&g, NodeIndex(0), vec![NodeIndex(2)]);
    astar2.run();
    assert_eq!(astar2.get_dist_to_to_node(NodeIndex(2)), Some(14.0));

    let mut astar3 = AStar::new(&g, NodeIndex(0), vec![NodeIndex(3)]);
    astar3.run();
    assert_eq!(astar3.get_dist_to_to_node(NodeIndex(3)), Some(17.0));

    let mut astar4 = AStar::new(&g, NodeIndex(0), vec![NodeIndex(4)]);
    astar4.run();
    assert_eq!(astar4.get_dist_to_to_node(NodeIndex(4)), Some(9.0));

    let mut astar5 = AStar::new(&g, NodeIndex(0), vec![NodeIndex(5)]);
    astar5.run();
    assert_eq!(astar5.get_dist_to_to_node(NodeIndex(5)), Some(13.0));

    let mut astar6 = AStar::new(&g, NodeIndex(0), vec![NodeIndex(6)]);
    astar6.run();
    assert_eq!(astar6.get_dist_to_to_node(NodeIndex(6)), Some(25.0));

    let mut astar7 = AStar::new(&g, NodeIndex(0), vec![NodeIndex(7)]);
    astar7.run();
    assert_eq!(astar7.get_dist_to_to_node(NodeIndex(7)), Some(8.0));
}

#[test]
fn osm_a_star_test() {
    // Load graph
    let g = load_pbf_file("test_data/osm/monaco-latest.osm.pbf").unwrap();
    // Get largest connected subgraph
    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();
    assert_eq!(c_g.num_vertices(), 15447);
    // Get start and end nodes
    let start_node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(LatLng {
        latitude: 43.7284765,
        longitude: 7.415138,
    });
    let end_node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(LatLng {
        latitude: 43.7341524,
        longitude: 7.4178794,
    });
    // Run A*
    let mut astar = AStar::new(&c_g, start_node_index, vec![end_node_index]);
    astar.run();
    // Reverse engineer shortest path
    let p = astar.get_graph_path(end_node_index).unwrap();
    // Check path length
    assert_eq!(p.get_length_on_graph(&c_g), 0.8388698268789816);
    // Check path points
    let mut points: Vec<(f64, f64)> = vec![];
    for node_index in &p.path {
        let node_data = c_g.nodes_data().get_node_data_by_index(*node_index);
        points.push(node_data.latlng.as_lat_lng_tuple())
    }
    assert_eq!(
        points,
        [
            (43.728444800000005, 7.4152383),
            (43.7285819, 7.415576000000001),
            (43.728604600000004, 7.4155176),
            (43.7286323, 7.4154615),
            (43.7286662, 7.4154137),
            (43.728715900000005, 7.415354000000001),
            (43.728739700000006, 7.415338200000001),
            (43.7287594, 7.4153137000000005),
            (43.7288147, 7.4153305000000005),
            (43.728862500000005, 7.415350900000001),
            (43.7289017, 7.415383200000001),
            (43.7289371, 7.4154279),
            (43.729076600000006, 7.415619400000001),
            (43.7291025, 7.415654900000001),
            (43.7294739, 7.4161647),
            (43.729504000000006, 7.416191100000001),
            (43.7295388, 7.416202500000001),
            (43.7295743, 7.4161976),
            (43.729606600000004, 7.416176900000001),
            (43.7296315, 7.416215),
            (43.729651600000004, 7.416224400000001),
            (43.729808600000005, 7.4164455),
            (43.729847400000004, 7.4165207),
            (43.729922300000005, 7.4165397),
            (43.7299335, 7.416545),
            (43.730154600000006, 7.4168478),
            (43.7304678, 7.417278100000001),
            (43.7304887, 7.4172522),
            (43.730541800000005, 7.4173232),
            (43.730628200000005, 7.417332900000001),
            (43.730662300000006, 7.4173941),
            (43.730675500000004, 7.417417100000001),
            (43.7307094, 7.4174762),
            (43.7307476, 7.4174723),
            (43.730778400000005, 7.4174733),
            (43.7308223, 7.4174734),
            (43.730968100000005, 7.417465900000001),
            (43.7310821, 7.41746),
            (43.731207600000005, 7.417453500000001),
            (43.7313965, 7.417417),
            (43.731576800000006, 7.4173821),
            (43.7316068, 7.417510900000001),
            (43.7316437, 7.4176719),
            (43.7319109, 7.4172199),
            (43.7318999, 7.4172731),
            (43.732241300000005, 7.417548300000001),
            (43.732620100000005, 7.4178537),
            (43.7327129, 7.417826700000001),
            (43.732727000000004, 7.417824),
            (43.7327358, 7.417829),
            (43.7327601, 7.4178429),
            (43.7328034, 7.4178696),
            (43.732825500000004, 7.417885600000001),
            (43.7328363, 7.4179034),
            (43.7328409, 7.417911),
            (43.732852, 7.4179249),
            (43.7328899, 7.417957400000001),
            (43.7330388, 7.4180511000000005),
            (43.7330185, 7.4181385),
            (43.7330246, 7.418144600000001),
            (43.7331208, 7.418124000000001),
            (43.733220800000005, 7.4181027),
            (43.733364900000005, 7.418072700000001),
            (43.733474400000006, 7.4180486000000005),
            (43.733576500000005, 7.4180269),
            (43.733683400000004, 7.4180041),
            (43.733760100000005, 7.417989),
            (43.7337775, 7.4181546),
            (43.733825100000004, 7.418147500000001),
            (43.7340816, 7.4180982),
            (43.734085400000005, 7.4181315),
            (43.734055600000005, 7.418137300000001),
            (43.7340517, 7.4180980000000005),
            (43.7340497, 7.4180524000000005),
            (43.734057500000006, 7.4179827000000005),
            (43.7340706, 7.417937),
            (43.7340967, 7.417902000000001),
            (43.734151700000005, 7.417887500000001)
        ]
    );
}

#[test]
fn dimacs_shortest_path_a_star_test() {
    // Load graph
    let dimacs_g = load_xgmml_file("test_data/dimacs_shortest_path/USA-road-d.NY.gr").unwrap();
    // Run A*
    let mut astar = AStar::new(&dimacs_g, NodeIndex(0), vec![NodeIndex(264345)]);
    astar.run();
    // Get shortest path length
    let p = astar.get_graph_path(NodeIndex(264345)).unwrap();
    // Check path length
    assert_eq!(p.get_length_on_graph(&dimacs_g), 495306.0);
}
