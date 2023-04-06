use crate::graphs::datastructures::{Digraph, GraphPath, NodeIndex};
use graphviz_rust::{
    cmd::{CommandArg, Format},
    dot_structures::Graph,
    exec, parse,
    printer::PrinterContext,
};

#[derive(Debug)]
struct GraphStringBody {
    graph_string: String,
}

fn graph_to_graphviz_body(g: &impl Digraph, color: &str, with_label: bool) -> GraphStringBody {
    let all_node_list: Vec<String> = (0..g.num_vertices()).map(|i| format!("{i}")).collect();
    let all_node_string = all_node_list.join("\n");
    let all_node_edges_list: Vec<String> = (0..g.num_vertices())
        .map(|i| {
            let edges_this_node_list: Vec<String> = g
                .adj(NodeIndex(i))
                .iter()
                .map(|adjacency| {
                    let label_str = if with_label {
                        format!(",headlabel=\"{:.2}\"", adjacency.weight)
                    } else {
                        "".to_string()
                    };
                    format!(
                        "{i} -> {}[color=\"{color}\"{label_str}]",
                        adjacency.node_index.0
                    )
                })
                .collect();
            edges_this_node_list.join("\n")
        })
        .collect();
    let all_node_edges_string = all_node_edges_list.join("\n");
    GraphStringBody {
        graph_string: format!("{all_node_string}\n{all_node_edges_string}"),
    }
}

fn path_to_graphviz_body(g: &impl Digraph, path: &GraphPath) -> GraphStringBody {
    let path_nodes_list: Vec<String> = (0..(path.path.len() - 1))
        .map(|i| {
            format!(
                "{} -> {}[headlabel=\"{}\", color=\"red\"]",
                path.path[i].0,
                path.path[i + 1].0,
                g.dist(path.path[i], path.path[i + 1])
            )
        })
        .collect();
    GraphStringBody {
        graph_string: path_nodes_list.join("\n"),
    }
}

fn graph_string_to_file(graph_string_body: GraphStringBody, output_path: &str) {
    // If you see an error like this
    //    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/graphs/output.rs:11:8
    // You need to install the graphviz package
    // https://graphviz.org/download/
    let graphviz_graph_string = &format!("digraph G {{\n{}\n}}", graph_string_body.graph_string);
    let graphviz_graph: Graph = parse(graphviz_graph_string).unwrap();

    exec(
        graphviz_graph,
        &mut PrinterContext::default(),
        vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output(output_path.to_string()),
            CommandArg::Custom("-Ksfdp".to_string()),
        ],
    )
    .unwrap();
}

/// Export just a Digraph to a GraphViz processed SVG file at the specified output_path
pub fn output_graph_to_file(g: &impl Digraph, output_path: &str) {
    graph_string_to_file(graph_to_graphviz_body(g, "black", true), output_path);
}

/// Export a Digraph and GraphPath to a GraphViz processed SVG file at the specified output_path
pub fn output_graph_to_file_with_path(g: &impl Digraph, path: &GraphPath, output_path: &str) {
    let graph_string_body = GraphStringBody {
        graph_string: format!(
            "{}\n{}",
            graph_to_graphviz_body(g, "transparent", false).graph_string,
            path_to_graphviz_body(g, path).graph_string
        ),
    };
    graph_string_to_file(graph_string_body, output_path);
}

#[cfg(test)]
mod tests {
    use crate::geography::datastructures::LatLng;
    use crate::graphs::datastructures::{ALDigraph, Digraph, GraphPath, NodeID, NodeIndex};
    use crate::graphs::output::graphviz::{graph_to_graphviz_body, path_to_graphviz_body};

    fn get_test_graph() -> ALDigraph {
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
}
