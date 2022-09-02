use rand::Rng;
use rand_distr::{Distribution, Normal};

use super::digraph;

pub fn get_random_graph(
    num_nodes: usize,
    conn_prob: f32,
    weight_mean: f32,
    weight_sd: f32,
) -> digraph::Digraph {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(weight_mean, weight_sd).unwrap();

    let mut g = digraph::Digraph::new(num_nodes);

    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i != j && (&mut rng).gen::<f32>() > conn_prob {
                let raw_weight = normal.sample(&mut rng);
                let rounded_weight = (raw_weight * 100.0).round() / 100.0;
                g.add_edge(i, j, rounded_weight);
            }
        }
    }

    g
}
