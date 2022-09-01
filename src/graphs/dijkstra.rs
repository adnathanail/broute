use std::collections::VecDeque;

use super::digraph;

#[cfg(test)]
#[path = "dijkstra_tests.rs"]
mod dijkstra_tests;

pub fn dijkstra(g: &digraph::Digraph) -> Vec<f32> {
    // Initialise all distances to infinity
    let mut dist_to = vec![f32::INFINITY; g.num_vertices as usize];
    dist_to[0] = 0.0;
    // Add all vertices to queue
    let mut queue = VecDeque::new();
    for v in 0..g.num_vertices {
        queue.push_back(v);
    }

    while !queue.is_empty() {
        // Find next closest node to the start
        let mut node_with_min_distance: usize = usize::MAX;
        let mut min_distance: f32 = f32::INFINITY;
        for i in &queue {
            if dist_to[*i] < min_distance {
                min_distance = dist_to[*i];
                node_with_min_distance = *i;
            }
        }
        // Take closest node, v, from queue
        let min_index = queue
            .iter()
            .position(|&r| r == node_with_min_distance)
            .unwrap();
        let v = queue.remove(min_index).unwrap();
        // Check every node, u, reachable from v
        //   to see if a route via v is shorter than the current shortest path
        for u in g.adj(v) {
            let alt = dist_to[v] + u.weight;
            if alt < dist_to[u.to] {
                dist_to[u.to] = alt;
            }
        }
    }
    dist_to
}
