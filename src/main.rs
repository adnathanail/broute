// Stop the compiler warning about unused functions
#![allow(dead_code)]

use std::fs;

use broute::graphs::datastructures::digraph::Digraph;
use broute::graphs::{
    output::output_graph_to_file_with_path,
    travelling_salesman::{get_path_length, travelling_salesman},
    tsplib::load_tsplib_file,
};

fn main() {
    let tsp_string = fs::read_to_string("test_data/dimacs_tsp/d1291.tsp").unwrap();

    println!("Reading file");

    let g = load_tsplib_file(tsp_string, usize::max_value());

    println!("Solving travelling salesman");

    let path = travelling_salesman(&g, true);

    println!("Final");

    println!("\t{:?}", path.path);
    println!("\t{}", get_path_length(&g, &path));
    path.path.iter().for_each(|x| {
        println!("\t{:?}", g.get_node_data(*x));
    });
    output_graph_to_file_with_path(&g, &path, "out/path_final.svg".to_string());

    println!("Done!");
}
