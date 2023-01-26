use super::*;

#[test]
fn connected_components_test() {
    let mut g = ALDigraph::new(5);

    for i in 0..5 {
        g.mut_nodes_data().add_node_data_by_parts(NodeID(i), 0.0, 0.0);
    }

    g.add_edge_by_id(NodeID(1), NodeID(0), 1.0);
    g.add_edge_by_id(NodeID(0), NodeID(2), 1.0);
    g.add_edge_by_id(NodeID(2), NodeID(1), 1.0);
    g.add_edge_by_id(NodeID(0), NodeID(3), 1.0);
    g.add_edge_by_id(NodeID(3), NodeID(4), 1.0);

    let mut cc = ConnectedComponents::new(&g);
    cc.run();

    assert_eq!(cc.cc, [2, 2, 2, 1, 0]);

    let components = cc.get_connected_components();

    assert_eq!(components[0].num_vertices(), 1);
    assert_eq!(components[0].adj(NodeIndex(0)).len(), 0);

    assert_eq!(components[1].num_vertices(), 1);
    assert_eq!(components[1].adj(NodeIndex(0)).len(), 0);

    println!("{:?}", components[2]);
    assert_eq!(components[2].num_vertices(), 3);
    assert_eq!(components[2].adj(NodeIndex(0)).len(), 1);
    assert_eq!(components[2].adj(NodeIndex(0))[0].node_index.0, 2);
    assert_eq!(components[2].adj(NodeIndex(1)).len(), 1);
    assert_eq!(components[2].adj(NodeIndex(1))[0].node_index.0, 0);
    assert_eq!(components[2].adj(NodeIndex(2)).len(), 1);
    assert_eq!(components[2].adj(NodeIndex(2))[0].node_index.0, 1);
}
