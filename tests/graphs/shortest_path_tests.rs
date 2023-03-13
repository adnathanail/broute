use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::{ConnectedComponents, Dijkstra};
use broute::graphs::datastructures;
use broute::graphs::datastructures::{Digraph, NodeID, NodeIndex};
use broute::graphs::input::{load_pbf_file, load_xgmml_file};

#[test]
fn simple_dijkstra_test() {
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

    let mut dj = Dijkstra::new(&g, NodeIndex(0), vec![NodeIndex(1)]);
    dj.run();
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(1)), Some(5.0));
    // We didn't ask for it, so A* won't have prioritised it, so the answer will be bad
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(2)), None);

    let mut dj = Dijkstra::new(&g, NodeIndex(0), vec![NodeIndex(2)]);
    dj.run();
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(2)), Some(14.0));

    let mut dj = Dijkstra::new(&g, NodeIndex(0), vec![NodeIndex(3)]);
    dj.run();
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(3)), Some(17.0));

    let mut dj = Dijkstra::new(&g, NodeIndex(0), vec![NodeIndex(4)]);
    dj.run();
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(4)), Some(9.0));

    let mut dj = Dijkstra::new(&g, NodeIndex(0), vec![NodeIndex(5)]);
    dj.run();
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(5)), Some(13.0));

    let mut dj = Dijkstra::new(&g, NodeIndex(0), vec![NodeIndex(6)]);
    dj.run();
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(6)), Some(25.0));

    let mut dj = Dijkstra::new(&g, NodeIndex(0), vec![NodeIndex(7)]);
    dj.run();
    assert_eq!(dj.get_dist_to_to_node(NodeIndex(7)), Some(8.0));
}

#[test]
fn osm_dijkstra_test() {
    // Load graph
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf").unwrap();
    // Get largest connected subgraph
    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();
    assert_eq!(c_g.num_vertices(), 9936);
    // Get start and end nodes
    let start_node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(LatLng {
        latitude: 43.7284765,
        longitude: 7.415138,
    });
    let end_node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(LatLng {
        latitude: 43.7341524,
        longitude: 7.4178794,
    });
    // Run Dijkstra
    let mut dj = Dijkstra::new(&c_g, start_node_index, vec![end_node_index]);
    dj.run();
    // Reverse engineer shortest path
    let p = dj.get_graph_path(end_node_index).unwrap();
    // Check path length
    assert_eq!(p.get_length_on_graph(&c_g), 1.416306768485833);
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
            (43.7282413, 7.4149406),
            (43.7279932, 7.414581200000001),
            (43.727894000000006, 7.4144215),
            (43.7279204, 7.414374100000001),
            (43.728048400000006, 7.414201),
            (43.7280669, 7.414170100000001),
            (43.728014800000004, 7.4140961),
            (43.7276101, 7.4135207),
            (43.7274216, 7.4132997000000005),
            (43.727468300000005, 7.4131885),
            (43.7275652, 7.412992200000001),
            (43.727580200000006, 7.4129687),
            (43.7276055, 7.4129768),
            (43.7279672, 7.4130849),
            (43.7280705, 7.4131444),
            (43.7281552, 7.4132313000000005),
            (43.7284565, 7.4136568),
            (43.728526200000005, 7.4137509),
            (43.7286918, 7.4139742),
            (43.728947500000004, 7.414333600000001),
            (43.7289873, 7.4143945),
            (43.7290323, 7.4143369),
            (43.729466900000006, 7.4137725),
            (43.729516700000005, 7.4137244),
            (43.7295328, 7.413725200000001),
            (43.729579, 7.413727400000001),
            (43.7296106, 7.4137823),
            (43.7297419, 7.4140101000000005),
            (43.730364300000005, 7.415119300000001),
            (43.730790600000006, 7.4158156),
            (43.7308906, 7.416007100000001),
            (43.7311373, 7.41643),
            (43.731330500000006, 7.416817600000001),
            (43.7314391, 7.417118),
            (43.7314765, 7.417188800000001),
            (43.731467200000004, 7.417214800000001),
            (43.7314616, 7.417242900000001),
            (43.731459900000004, 7.417271800000001),
            (43.7314621, 7.4173007),
            (43.7314682, 7.417328500000001),
            (43.7314779, 7.4173543),
            (43.731490900000004, 7.4173771),
            (43.731506700000004, 7.4173961),
            (43.7315248, 7.417410800000001),
            (43.7315446, 7.4174205),
            (43.7315654, 7.4174249),
            (43.731586300000004, 7.4174240000000005),
            (43.7316068, 7.4174176),
            (43.7316261, 7.417406000000001),
            (43.731643500000004, 7.4173897),
            (43.7316902, 7.417461100000001),
            (43.731745100000005, 7.417548300000001),
            (43.731782900000006, 7.417625800000001),
            (43.731811900000004, 7.4176953),
            (43.7318525, 7.417808900000001),
            (43.7319103, 7.417967600000001),
            (43.7319727, 7.418122800000001),
            (43.7320397, 7.4182744000000005),
            (43.7320706, 7.4182394),
            (43.7320907, 7.418259900000001),
            (43.7321006, 7.4182607),
            (43.732116500000004, 7.418317300000001),
            (43.732148800000004, 7.4183741),
            (43.732148200000005, 7.418391300000001),
            (43.732409100000005, 7.418774300000001),
            (43.7324258, 7.4187865),
            (43.7324344, 7.418780600000001),
            (43.732465100000006, 7.418770100000001),
            (43.732516100000005, 7.418627300000001),
            (43.7326471, 7.4180991),
            (43.7326843, 7.418118300000001),
            (43.732722, 7.4181417000000005),
            (43.732756200000004, 7.418159200000001),
            (43.7327239, 7.4183079),
            (43.7326221, 7.4186759),
            (43.732597500000004, 7.418755900000001),
            (43.7325911, 7.4188178),
            (43.7326065, 7.4188426000000005),
            (43.732635900000005, 7.418858500000001),
            (43.732675300000004, 7.4188399),
            (43.732711, 7.418823000000001),
            (43.7327084, 7.4187865),
            (43.7327615, 7.4185748),
            (43.732814100000006, 7.4184691),
            (43.7328727, 7.418403400000001),
            (43.7329596, 7.418358400000001),
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
fn dimacs_shortest_path_dijkstra_test() {
    // Load graph
    let dimacs_g = load_xgmml_file("test_data/dimacs_shortest_path/USA-road-d.NY.gr").unwrap();
    // Run Dijkstra
    let mut dj = Dijkstra::new(&dimacs_g, NodeIndex(0), vec![NodeIndex(264345)]);
    dj.run();
    // Get shortest path length
    let p = dj.get_graph_path(NodeIndex(264345)).unwrap();
    // Check path length
    assert_eq!(p.get_length_on_graph(&dimacs_g), 495306.0);
}
