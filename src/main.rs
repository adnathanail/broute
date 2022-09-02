// Stop the compiler warning about unused functions
#![allow(dead_code)]

use broute::graphs;

fn main() {
    let g = graphs::random_graph::get_random_graph(5, 0.5, 4.0, 1.0);

    println!("{}", g);

    println!("{:?}", graphs::dijkstra::dijkstra(&g));
}
