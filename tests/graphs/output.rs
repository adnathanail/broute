use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::ConnectedComponents;
use broute::graphs::datastructures::{ALDigraph, Digraph, GraphPath, NodeID, NodeIndex};
use broute::graphs::output::{
    graph_to_graphviz_body, output_graph_to_file, output_graph_to_file_with_path,
    path_to_graphviz_body, to_svg,
};
use std::fs;

fn get_test_graph() -> ALDigraph {
    let mut g = ALDigraph::new(5);

    g.mut_nodes_data().add_node_data_by_parts(
        NodeID(0),
        LatLng {
            latitude: 1.0,
            longitude: 0.0,
        },
    );

    g.mut_nodes_data().add_node_data_by_parts(
        NodeID(1),
        LatLng {
            latitude: 0.0,
            longitude: 1.0,
        },
    );

    g.mut_nodes_data().add_node_data_by_parts(
        NodeID(2),
        LatLng {
            latitude: 2.0,
            longitude: 1.0,
        },
    );

    g.mut_nodes_data().add_node_data_by_parts(
        NodeID(3),
        LatLng {
            latitude: 0.0,
            longitude: 3.0,
        },
    );

    g.mut_nodes_data().add_node_data_by_parts(
        NodeID(4),
        LatLng {
            latitude: 3.0,
            longitude: 2.0,
        },
    );

    g.add_edge_by_id(NodeID(1), NodeID(0), 1.0);
    g.add_edge_by_id(NodeID(0), NodeID(2), 1.0);
    g.add_edge_by_id(NodeID(2), NodeID(1), 1.0);
    g.add_edge_by_id(NodeID(0), NodeID(3), 1.0);
    g.add_edge_by_id(NodeID(3), NodeID(4), 1.0);

    g
}

fn get_test_graph_path() -> GraphPath {
    GraphPath {
        path: vec![
            NodeIndex(1),
            NodeIndex(0),
            NodeIndex(2),
            NodeIndex(1),
            NodeIndex(0),
            NodeIndex(3),
            NodeIndex(4),
        ],
    }
}

#[test]
fn graph_to_graphviz_body_test() {
    assert_eq!(
        graph_to_graphviz_body(&get_test_graph(), "black", true).graph_string,
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
}

#[test]
fn path_to_graphviz_body_test() {
    assert_eq!(
        path_to_graphviz_body(&get_test_graph(), &get_test_graph_path()).graph_string,
        r#"1 -> 0[headlabel="1", color="red"]
0 -> 2[headlabel="1", color="red"]
2 -> 1[headlabel="1", color="red"]
1 -> 0[headlabel="1", color="red"]
0 -> 3[headlabel="1", color="red"]
3 -> 4[headlabel="1", color="red"]"#
    );
}

#[test]
fn output_graph_to_file_test() {
    output_graph_to_file(&get_test_graph(), "out/output_graph_to_file_test.svg");
    assert!(fs::read_to_string("out/output_graph_to_file_test.svg").unwrap().contains("M161.74,-213.4C172.33,-213.86 181.3,-211.63 181.3,-206.71 181.3,-203.71 177.97,-201.71 172.97,-200.71"));
}

#[test]
fn output_graph_to_file_with_path_test() {
    output_graph_to_file_with_path(
        &get_test_graph(),
        &get_test_graph_path(),
        "out/output_graph_to_file_with_path_test.svg",
    );
    assert!(
        fs::read_to_string("out/output_graph_to_file_with_path_test.svg")
            .unwrap()
            .contains("M140,-81.79C116.08,-89.86 80.87,-104.46 56.12,-116.45")
    );
}

#[test]
fn to_svg_test() {
    to_svg(
        &get_test_graph(),
        &get_test_graph_path(),
        "out/to_svg_test.svg",
    );
    assert!(fs::read_to_string("out/to_svg_test.svg")
        .unwrap()
        .contains("M33.333332,0 L0,33.333332 L33.333332,66.666664 L33.333332,0 L0,33.333332 L100,0 L66.666664,100 L33.333332,0"));
}
