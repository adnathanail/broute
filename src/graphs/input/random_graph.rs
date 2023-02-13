use crate::graphs::datastructures::{AMDigraph, Digraph, LatLng, NodeID};
use rand::Rng;
use rand_distr::{Distribution, Normal};

pub fn get_random_graph(
    num_nodes: usize,
    conn_prob: f64,
    weight_mean: f64,
    weight_sd: f64,
) -> AMDigraph {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(weight_mean, weight_sd).unwrap();

    let mut g = AMDigraph::new(num_nodes);

    for i in 0..num_nodes {
        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(i),
            LatLng {
                latitude: 0.0,
                longitude: 0.0,
            },
        )
    }

    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i != j && rng.gen::<f64>() > conn_prob {
                let raw_weight = normal.sample(&mut rng);
                let rounded_weight = (raw_weight * 100.0).round() / 100.0;
                g.add_edge_by_id(NodeID(i), NodeID(j), rounded_weight);
            }
        }
    }

    g
}
