use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::ConnectedComponents;
use broute::graphs::datastructures::{ALDigraph, Digraph, GraphPath, NodeID, NodeIndex};
use broute::graphs::output::{graph_to_graphviz_body, output_graph_to_file, path_to_graphviz_body};

#[test]
fn xyz_to_graphviz_body_test() {
    let mut g = ALDigraph::new(5);

    for i in 0..5 {
        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(i),
            LatLng {
                latitude: 0.0,
                longitude: 0.0,
            },
        );
    }

    g.add_edge_by_id(NodeID(1), NodeID(0), 1.0);
    g.add_edge_by_id(NodeID(0), NodeID(2), 1.0);
    g.add_edge_by_id(NodeID(2), NodeID(1), 1.0);
    g.add_edge_by_id(NodeID(0), NodeID(3), 1.0);
    g.add_edge_by_id(NodeID(3), NodeID(4), 1.0);

    assert_eq!(
        graph_to_graphviz_body(&g, "black", true).graph_string,
        r#"0
1
2
3
4
0 -> 0[color="black",headlabel="1"]
0 -> 1[color="black",headlabel="1"]
1 -> 0[color="black",headlabel="1"]
2 -> 0[color="black",headlabel="1"]
3 -> 0[color="black",headlabel="1"]
"#
    );

    let p = GraphPath {
        path: vec![
            NodeIndex(1),
            NodeIndex(0),
            NodeIndex(2),
            NodeIndex(1),
            NodeIndex(0),
            NodeIndex(3),
            NodeIndex(4),
        ],
    };

    assert_eq!(
        path_to_graphviz_body(&g, &p).graph_string,
        r#"1 -> 0[headlabel="1", color="red"]
0 -> 2[headlabel="1", color="red"]
2 -> 1[headlabel="1", color="red"]
1 -> 0[headlabel="1", color="red"]
0 -> 3[headlabel="1", color="red"]
3 -> 4[headlabel="1", color="red"]"#
    );
}
