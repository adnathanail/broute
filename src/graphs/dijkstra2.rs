use crate::algorithms::priority_queue::PriorityQueue;

use super::digraph;

#[cfg(test)]
#[path = "dijkstra_tests.rs"]
mod dijkstra_tests;

pub fn dijkstra2(g: &digraph::Digraph) -> Vec<f32> {
    // Initialise all distances to infinity
    let mut dist_to = vec![f32::INFINITY; g.num_vertices as usize];
    dist_to[0] = 0.0;
    // Add first vertex to queue
    let mut queue = PriorityQueue::new();
    queue.push(0.0, 0);

    // Take next closest node from the heap
    while let Some((v, cost)) = queue.pop() {
        // Short circuit
        if cost > dist_to[v] { continue; }

        // Check every node, u, reachable from v
        //   to see if a route via v is shorter than the current shortest path
        for u in g.adj(v) {
            let alt = dist_to[v] + u.weight;
            if alt < dist_to[u.to] {
                // Add adjacent node to queue
                queue.push(alt, u.to);
                dist_to[u.to] = alt;
            }
        }
    }
    dist_to
}
