use rand::thread_rng;
use rand::seq::SliceRandom;

use super::digraph::Digraph;

#[derive(Debug)]
pub struct GraphPath {
    pub path: Vec<usize>,
}

pub fn travelling_salesman(g: &Digraph) -> GraphPath {
    let mut path: Vec<usize> = (0..g.num_vertices).collect();
    path.shuffle(&mut thread_rng());
    GraphPath{ path: path }
}

pub fn get_path_length(g: &Digraph, path: &GraphPath) -> f32 {
    (0..(path.path.len() -1)).fold(0f32, |total, i| {
        total + g.dist(path.path[i], path.path[i+1])
    })
}