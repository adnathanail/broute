#![allow(dead_code)]

mod practice;
mod graph;

fn main() {
    let mut g = graph::Graph::new(10);

    println!("{:?}", g);

    g.add_edge(1, 3);

    println!("{:?}", g);
}
