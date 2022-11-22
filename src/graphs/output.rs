use graphviz_rust::{
    cmd::{CommandArg, Format},
    dot_structures::Graph,
    exec, parse,
    printer::PrinterContext,
};

use super::digraph::Digraph;

pub fn output_graph_to_file(g: &Digraph, output_path: String) {
    let all_node_list: Vec<String> = (0..g.num_vertices).map(|i| format!("{}", i)).collect();
    let all_node_string = all_node_list.join("\n");
    let all_node_edges_list: Vec<String> = (0..g.num_vertices).map(|i| {
        let edges_this_node_list: Vec<String> = g.adj(i).iter().map(|edge| {
            format!("{} -> {}[headlabel=\"{}\"]", i, edge.to, edge.weight)
        }).collect();
        edges_this_node_list.join("\n")
    }).collect();
    let all_node_edges_string = all_node_edges_list.join("\n");
    let graph_string = format!("digraph G {{\n{}\n{}\n}}", all_node_string, all_node_edges_string);

    let graphviz_graph: Graph = parse(&graph_string).unwrap();

    // If you see an error like this
    //    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/graphs/output.rs:11:8
    // You need to install the graphviz package
    // https://graphviz.org/download/
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
