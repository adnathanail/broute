use crate::algorithms::haversine::haversine;
use crate::graphs::datastructures::al_digraph::ALDigraph;
use crate::graphs::datastructures::digraph::{Digraph, NodeID};
use osmpbf::{Element, ElementReader};

pub fn load_pbf_file(pbf_path: &str) -> ALDigraph {
    let reader = ElementReader::from_path(pbf_path).unwrap();

    let num_nodes = reader
        .par_map_reduce(
            |element| match element {
                Element::Node(_) => 1,
                Element::DenseNode(_) => 1,
                _ => 0,
            },
            || 0_u64,     // Zero is the identity value for addition
            |a, b| a + b, // Sum the partial results
        )
        .unwrap();

    println!("Number of nodes: {num_nodes}");

    let mut g = ALDigraph::new(num_nodes as usize);

    println!("Graph initialised");

    let reader = ElementReader::from_path(pbf_path).unwrap();

    reader
        .for_each(|element| {
            if let Element::Node(n) = element {
                g.mut_nodes_data()
                    .add_node_data_by_parts(NodeID(n.id() as usize), n.lat(), n.lon())
            } else if let Element::DenseNode(dn) = element {
                g.mut_nodes_data().add_node_data_by_parts(
                    NodeID(dn.id() as usize),
                    dn.lat(),
                    dn.lon(),
                )
            }
        })
        .unwrap();

    println!("Nodes added");

    let mut ways = 0_u64;

    let reader = ElementReader::from_path(pbf_path).unwrap();

    reader
        .for_each(|element| {
            if let Element::Way(w) = element {
                let node_ids = w.refs().collect::<Vec<_>>();
                for i in 0..node_ids.len() - 1 {
                    let from_id = NodeID(node_ids[i] as usize);
                    let from_node_data = g.nodes_data().get_node_data_by_id(from_id);

                    let to_id = NodeID(node_ids[i + 1] as usize);
                    let to_node_data = g.nodes_data().get_node_data_by_id(to_id);

                    let weight = haversine(
                        from_node_data.longitude,
                        from_node_data.latitude,
                        to_node_data.longitude,
                        to_node_data.latitude,
                    );
                    g.add_edge_by_id(
                        NodeID(node_ids[i] as usize),
                        NodeID(node_ids[i + 1] as usize),
                        weight,
                    );
                }
                ways += 1;
            }
        })
        .unwrap();

    println!("Edges added");

    g
}
