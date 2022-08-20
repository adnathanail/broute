use super::digraph;
use std::collections::VecDeque;

#[cfg(test)]
#[path = "dijkstra_tests.rs"]
mod dijkstra_tests;

pub fn dijkstra(g: &digraph::Digraph) -> Vec<f32> {
    let mut dist_to = vec![f32::INFINITY; g.num_vertices as usize];
    dist_to[0] = 0.0;
    let mut queue = VecDeque::new();
    for v in 0..g.num_vertices {
        queue.push_back(v);
    }

    while !queue.is_empty() {
        let mut node_with_min_distance: usize = usize::MAX;
        let mut min_distance: f32 = f32::INFINITY;
        for i in &queue {
            if dist_to[*i] < min_distance {
                min_distance = dist_to[*i];
                node_with_min_distance = *i;
            }
        }
        let min_index = queue
            .iter()
            .position(|&r| r == node_with_min_distance)
            .unwrap();
        let v = queue.remove(min_index).unwrap();
        for u in g.adj(v) {
            let alt = dist_to[v] + u.weight;
            if alt < dist_to[u.to] {
                dist_to[u.to] = alt;
            }
        }
    }
    dist_to
}
