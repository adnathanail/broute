// Stop the compiler warning about unused functions
#![allow(dead_code)]

use broute::graphs;

fn main() {
    let mut g = graphs::digraph::Digraph::new(8);

    g.add_edge(0, 2, 0.26);
    g.add_edge(0, 4, 0.38);
    g.add_edge(1, 3, 0.29);
    g.add_edge(2, 7, 0.34);
    g.add_edge(3, 6, 0.52);
    g.add_edge(4, 5, 0.35);
    g.add_edge(4, 7, 0.37);
    g.add_edge(5, 1, 0.32);
    g.add_edge(5, 4, 0.35);
    g.add_edge(5, 7, 0.28);
    g.add_edge(6, 0, 0.58);
    g.add_edge(6, 2, 0.40);
    g.add_edge(6, 4, 0.93);
    g.add_edge(7, 3, 0.39);
    g.add_edge(7, 5, 0.28);

    println!("{}", g);

    graphs::dijkstra::dijkstra(&g);
    graphs::dijkstra::dijkstra(&g);
}
