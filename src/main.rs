// Stop the compiler warning about unused functions
#![allow(dead_code)]

mod graph;
mod practice;

fn main() {
    let mut g = graph::Graph::new(13);

    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(0, 5);
    g.add_edge(0, 6);
    g.add_edge(3, 4);
    g.add_edge(3, 5);
    g.add_edge(4, 5);
    g.add_edge(4, 6);

    g.add_edge(7, 8);

    g.add_edge(9, 10);
    g.add_edge(9, 11);
    g.add_edge(9, 12);
    g.add_edge(11, 12);

    println!("{:?}", g);

    let mut out: Vec<Vec<i32>> = vec![];
    g.connected_components(&mut out);
    println!("{:?}", out);
}
