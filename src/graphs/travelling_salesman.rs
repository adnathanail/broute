use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::graphs::output::output_graph_to_file_with_path;

use super::digraph::Digraph;

#[derive(Debug, Clone)]
pub struct GraphPath {
    pub path: Vec<usize>,
}

pub fn travelling_salesman(g: &Digraph) -> GraphPath {
    let mut rng = thread_rng();

    let mut path = GraphPath {
        path: (0..g.num_vertices).collect(),
    };
    path.path.shuffle(&mut rng);
    let mut path_length = get_path_length(g, &path);

    println!("Initial state");

    println!("\t{:?}", path.path);
    println!("\t{}", get_path_length(&g, &path));

    let mut temp = f32::sqrt(g.num_vertices as f32);
    let mut iterations = 0;
    while temp > 1e-8_f32 && iterations < (100 * g.num_vertices){
        let mut potential_new_path = path.clone();

        let node_index_to_mutate = rng.gen_range(0..(g.num_vertices - 1));

        let reverse_or_transport: bool = rng.gen();

        if reverse_or_transport {
            let node_index_to_swap_with = if node_index_to_mutate < (g.num_vertices - 1) {
                node_index_to_mutate + 1
            } else {
                0
            };
            let swap = potential_new_path.path[node_index_to_mutate];
            potential_new_path.path[node_index_to_mutate] =
                potential_new_path.path[node_index_to_swap_with];
            potential_new_path.path[node_index_to_swap_with] = swap;
        } else {
            let node_to_move = potential_new_path.path[node_index_to_mutate];
            // -2 because we are looking for new position with 1 node missing
            let new_node_position = rng.gen_range(0..(g.num_vertices - 2));
            potential_new_path.path.remove(node_index_to_mutate);
            potential_new_path
                .path
                .insert(new_node_position, node_to_move)
        }
        let new_path_length = get_path_length(g, &potential_new_path);
        if new_path_length < path_length {
            path = potential_new_path;
            path_length = new_path_length;
        } else {
            if f32::exp(- f32::abs(new_path_length - path_length) / temp) > rng.gen::<f32>() {
                path = potential_new_path;
                path_length = new_path_length;
            }
        }

        temp *= 0.995;
        iterations += 1;
    }
    path
}

pub fn get_path_length(g: &Digraph, path: &GraphPath) -> f32 {
    (0..(path.path.len() - 1)).fold(0f32, |total, i| {
        total + g.dist(path.path[i], path.path[i + 1])
    })
}
