// Stop the compiler warning about unused functions
#![allow(dead_code)]

use std::fs;

use broute::graphs::{tsplib::load_tsplib_file, output::{output_graph_to_file, output_graph_to_file_with_path}, dijkstra::dijkstra, travelling_salesman::{travelling_salesman, get_path_length}};

fn main() {
    let tsp_string = fs::read_to_string("test_data/dimacs_tsp/d1291.tsp").unwrap();

    println!("Reading file");

    let g = load_tsplib_file(tsp_string, 5);

    output_graph_to_file(&g, "out/graph.svg".to_string());

    println!("Running Dijkstra");

    println!("{:?}", dijkstra(&g));

    println!("Solving travelling salesman");

    let path = travelling_salesman(&g);

    println!("{:?}", path);

    println!("Getting path length");

    println!("{}", get_path_length(&g, &path));

    println!("Visualising path");

    output_graph_to_file_with_path(&g, &path, "out/path.svg".to_string());

    println!("Done!");
}
