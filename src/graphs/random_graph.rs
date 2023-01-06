use rand::Rng;
use rand_distr::{Distribution, Normal};

use super::datastructures::am_digraph::AMDigraph;
use super::datastructures::digraph::Digraph;

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
        for j in 0..num_nodes {
            if i != j && rng.gen::<f64>() > conn_prob {
                let raw_weight = normal.sample(&mut rng);
                let rounded_weight = (raw_weight * 100.0).round() / 100.0;
                g.add_edge(i, j, rounded_weight);
            }
        }
    }

    g
}
