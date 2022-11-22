use graphviz_rust::{
    cmd::{CommandArg, Format},
    dot_structures::Graph,
    exec, parse,
    printer::PrinterContext,
};

use super::digraph::Digraph;

pub fn output_graph_to_file(g: &Digraph, output_path: String) {
    let g: Graph = parse(&g.get_graphviz_string()).unwrap();

    // If you see an error like this
    //    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/graphs/output.rs:11:8
    // You need to install the graphviz package
    // https://graphviz.org/download/
    exec(
        g,
        &mut PrinterContext::default(),
        vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output(output_path),
            CommandArg::Custom("-Ksfdp".to_string()),
        ],
    )
    .unwrap();
}
