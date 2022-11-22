// Stop the compiler warning about unused functions
#![allow(dead_code)]

use std::fs;

use broute::graphs::{self, output::output_graph_to_file};

fn main() {
    let tsp_string = fs::read_to_string("test_data/dimacs_tsp/d1291.tsp").unwrap();

    println!("Reading file");

    let g = graphs::tsplib::load_tsplib_file(tsp_string, 10);

    println!("Generating visual graph");

    output_graph_to_file(&g, "out/graph.svg".to_string());

    println!("Running Dijkstra");

    println!("{:?}", graphs::dijkstra::dijkstra(&g));

    println!("Done!");
}
