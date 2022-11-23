use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};


use crate::graphs::output::output_graph_to_file_with_path;

use super::digraph::Digraph;

#[derive(Debug)]
pub struct GraphPath {
    pub path: Vec<usize>,
}

pub fn travelling_salesman(g: &Digraph) -> GraphPath {
    let mut rng = thread_rng();

    let mut path = GraphPath{ path: (0..g.num_vertices).collect()};
    path.path.shuffle(&mut rng);

    println!("Initial");

    let mut temp = 0.5;
    while temp > 0.3 {
        println!("\t{:?}", path.path);
        println!("\t{}", get_path_length(g, &path));

        println!("{}", temp);
        let node_index_to_mutate = rng.gen_range(0..(g.num_vertices - 1));

        let reverse_or_transport: bool = rng.gen();

        if reverse_or_transport {
            println!("\tReverse");
            let node_index_to_swap_with =
                if node_index_to_mutate < (g.num_vertices - 1) {
                    node_index_to_mutate + 1
                } else {
                    0
                };
            println!("\t{}->{}", path.path[node_index_to_mutate], path.path[node_index_to_swap_with]);
            let swap = path.path[node_index_to_mutate];
            path.path[node_index_to_mutate] = path.path[node_index_to_swap_with];
            path.path[node_index_to_swap_with] = swap;
        } else {
            println!("\tTransport");
            let node_to_move = path.path[node_index_to_mutate];
            // -2 because we are looking for new position with 1 node missing
            let new_node_position = rng.gen_range(0..(g.num_vertices - 2));
            println!("\t{} to index {}", node_to_move, new_node_position);
            path.path.remove(node_index_to_mutate);
            path.path.insert(new_node_position, node_to_move)
        }

        output_graph_to_file_with_path(&g, &path, format!("out/path_{}.svg", temp));
        temp *= 0.9;
    }
    path
}

pub fn get_path_length(g: &Digraph, path: &GraphPath) -> f32 {
    (0..(path.path.len() -1)).fold(0f32, |total, i| {
        total + g.dist(path.path[i], path.path[i+1])
    })
}