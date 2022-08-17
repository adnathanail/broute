#![allow(dead_code)]

mod practice;
mod graph;

fn main() {
    let g = graph::Graph::new(10);

    println!("{:?}", g);
}
