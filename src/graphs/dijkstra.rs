use super::super::algorithms::priority_queue::PriorityQueue;
use crate::graphs::datastructures::digraph::Digraph;
use std::collections::VecDeque;

#[cfg(test)]
#[path = "dijkstra_tests.rs"]
mod dijkstra_tests;

pub fn dijkstra(g: &dyn Digraph) -> Vec<f64> {
    // Initialise all distances to infinity
    let mut dist_to = vec![f64::INFINITY; g.num_vertices()];
    dist_to[0] = 0.0;
    // Add all vertices to queue
    let mut queue = VecDeque::new();
    for v in 0..g.num_vertices() {
        queue.push_back(v);
    }

    while !queue.is_empty() {
        // Find next closest node to the start
        let mut node_with_min_distance: usize = usize::MAX;
        let mut min_distance: f64 = f64::INFINITY;
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
        for adjacency in g.adj(v).iter() {
            let alt = dist_to[v] + adjacency.weight;
            if alt < dist_to[adjacency.to] {
                dist_to[adjacency.to] = alt;
            }
        }
    }
    dist_to
}

pub fn dijkstra2(g: &dyn Digraph) -> Vec<f64> {
    // Initialise all distances to infinity
    let mut dist_to = vec![f64::INFINITY; g.num_vertices()];
    dist_to[0] = 0.0;
    // Add first vertex to queue
    let mut queue = PriorityQueue::new();
    queue.push(0.0, 0);

    // Take next closest node from the heap
    while let Some((v, cost)) = queue.pop() {
        // Short circuit
        if cost > dist_to[v] {
            continue;
        }

        // Check every node, u, reachable from v
        //   to see if a route via v is shorter than the current shortest path
        for adjacency in g.adj(v).iter() {
            let alt = dist_to[v] + adjacency.weight;
            if alt < dist_to[adjacency.to] {
                // Add adjacent node to queue
                queue.push(alt, adjacency.to);
                dist_to[adjacency.to] = alt;
            }
        }
    }
    dist_to
}
