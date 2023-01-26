use super::*;
use crate::graphs::datastructures;
use crate::graphs::datastructures::digraph::NodeID;

#[test]
fn dijkstra_test() {
    let mut g = datastructures::am_digraph::AMDigraph::new(8);

    for i in 0..8 {
        g.mut_nodes_data().add_node_data_by_parts(NodeID(i), 0.0, 0.0);
    }

    g.add_edge_by_id(NodeID(0), NodeID(1), 5.0);
    g.add_edge_by_id(NodeID(0), NodeID(4), 9.0);
    g.add_edge_by_id(NodeID(0), NodeID(7), 8.0);
    g.add_edge_by_id(NodeID(1), NodeID(2), 12.0);
    g.add_edge_by_id(NodeID(1), NodeID(3), 15.0);
    g.add_edge_by_id(NodeID(1), NodeID(7), 4.0);
    g.add_edge_by_id(NodeID(2), NodeID(3), 3.0);
    g.add_edge_by_id(NodeID(2), NodeID(6), 11.0);
    g.add_edge_by_id(NodeID(3), NodeID(6), 9.0);
    g.add_edge_by_id(NodeID(4), NodeID(5), 4.0);
    g.add_edge_by_id(NodeID(4), NodeID(6), 20.0);
    g.add_edge_by_id(NodeID(4), NodeID(7), 5.0);
    g.add_edge_by_id(NodeID(5), NodeID(2), 1.0);
    g.add_edge_by_id(NodeID(5), NodeID(6), 13.0);
    g.add_edge_by_id(NodeID(7), NodeID(2), 7.0);
    g.add_edge_by_id(NodeID(7), NodeID(5), 6.0);

    assert_eq!(
        dijkstra(&g, NodeIndex(0)).0,
        [0.0, 5.0, 14.0, 17.0, 9.0, 13.0, 25.0, 8.0]
    );
    assert_eq!(
        dijkstra2(&g, NodeIndex(0)),
        [0.0, 5.0, 14.0, 17.0, 9.0, 13.0, 25.0, 8.0]
    );
}
