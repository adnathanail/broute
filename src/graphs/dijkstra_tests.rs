use super::*;
use crate::graphs::datastructures;

#[test]
fn dijkstra_test() {
    let mut g = datastructures::am_digraph::AMDigraph::new(8);

    for i in 0..8 {
        g.add_node_data(i, 0.0, 0.0);
    }

    g.add_edge(0, 1, 5.0);
    g.add_edge(0, 4, 9.0);
    g.add_edge(0, 7, 8.0);
    g.add_edge(1, 2, 12.0);
    g.add_edge(1, 3, 15.0);
    g.add_edge(1, 7, 4.0);
    g.add_edge(2, 3, 3.0);
    g.add_edge(2, 6, 11.0);
    g.add_edge(3, 6, 9.0);
    g.add_edge(4, 5, 4.0);
    g.add_edge(4, 6, 20.0);
    g.add_edge(4, 7, 5.0);
    g.add_edge(5, 2, 1.0);
    g.add_edge(5, 6, 13.0);
    g.add_edge(7, 2, 7.0);
    g.add_edge(7, 5, 6.0);

    assert_eq!(dijkstra(&g), [0.0, 5.0, 14.0, 17.0, 9.0, 13.0, 25.0, 8.0]);
    assert_eq!(dijkstra2(&g), [0.0, 5.0, 14.0, 17.0, 9.0, 13.0, 25.0, 8.0]);
}
