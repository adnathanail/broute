// Stop the compiler warning about unused functions
#![allow(dead_code)]

use std::fs;

use broute::graphs::datastructures::digraph::Digraph;
use broute::graphs::{
    algorithms::travelling_salesman::{get_path_length, travelling_salesman},
    input::tsplib::load_tsplib_file,
    output::graphviz::output_graph_to_file_with_path,
};

fn main() {
    let tsp_string = fs::read_to_string("test_data/dimacs_tsp/d1291.tsp").unwrap();

    println!("Reading file");

    let g = load_tsplib_file(tsp_string, usize::MAX);

    println!("Solving travelling salesman");

    let path = travelling_salesman(&g, true);

    println!("Final");

    println!("\t{:?}", path.path);
    println!("\t{}", get_path_length(&g, &path));
    path.path.iter().for_each(|x| {
        println!("\t{:?}", g.nodes_data().get_node_data_by_index(*x));
    });
    output_graph_to_file_with_path(&g, &path, "out/path_final.svg".to_string());

    println!("Done!");
}
