use graphviz_rust::{cmd::{CommandArg, Format}, exec, printer::PrinterContext, parse, dot_structures::Graph};

use super::{digraph::Digraph, travelling_salesman::GraphPath};

#[derive(Debug)]
struct GraphStringBody {
    graph_string: String,
}

fn graph_to_graphviz_body(g: &Digraph) -> GraphStringBody {
    let all_node_list: Vec<String> = (0..g.num_vertices).map(|i| format!("{}", i)).collect();
    let all_node_string = all_node_list.join("\n");
    let all_node_edges_list: Vec<String> = (0..g.num_vertices).map(|i| {
        let edges_this_node_list: Vec<String> = g.adj(i).iter().map(|edge| {
            format!("{} -> {}[headlabel=\"{}\"]", i, edge.to, edge.weight)
        }).collect();
        edges_this_node_list.join("\n")
    }).collect();
    let all_node_edges_string = all_node_edges_list.join("\n");
    GraphStringBody{ graph_string: format!("{}\n{}", all_node_string, all_node_edges_string)}
}

fn path_to_graphviz_body(path: &GraphPath) -> GraphStringBody {
    let path_nodes_list: Vec<String> = (0..(path.path.len() -1)).map(|i| {
        format!("{} -> {}[color=\"red\"]", path.path[i], path.path[i+1])
    }).collect();
    GraphStringBody{ graph_string: path_nodes_list.join("\n")}
}

fn graph_string_to_file(graph_string_body: GraphStringBody, output_path: String) {
    // If you see an error like this
    //    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/graphs/output.rs:11:8
    // You need to install the graphviz package
    // https://graphviz.org/download/
    let graphviz_graph: Graph = parse(&format!("digraph G {{\n{}\n}}", graph_string_body.graph_string)).unwrap();

    exec(
            graphviz_graph,
        &mut PrinterContext::default(),
        vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output(output_path),
            CommandArg::Custom("-Ksfdp".to_string()),
        ],
    )
    .unwrap();
}

pub fn output_graph_to_file(g: &Digraph, output_path: String) {
    graph_string_to_file(graph_to_graphviz_body(g), output_path);
}

pub fn output_graph_to_file_with_path(g: &Digraph, path: &GraphPath, output_path: String) {
    let graph_string_body = GraphStringBody{
        graph_string: format!("{}\n{}", graph_to_graphviz_body(g).graph_string, path_to_graphviz_body(path).graph_string)
    };
    graph_string_to_file(graph_string_body, output_path);
}