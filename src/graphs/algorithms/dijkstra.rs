use super::super::super::algorithms::priority_queue::PriorityQueue;
use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};

#[cfg(test)]
#[path = "dijkstra_tests.rs"]
mod dijkstra_tests;

pub fn dijkstra(g: &dyn Digraph, from_node: NodeIndex) -> (Vec<f64>, Vec<Option<usize>>) {
    // Initialise all distances to infinity
    let mut dist_to = vec![f64::INFINITY; g.num_vertices()];
    dist_to[from_node.0] = 0.0;
    // Initialise all parents to none
    let mut parent = vec![None; g.num_vertices()];
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
        for adjacency in g.adj(NodeIndex(v)).iter() {
            let alt = dist_to[v] + adjacency.weight;
            if alt < dist_to[adjacency.node_index.0] {
                // Add adjacent node to queue
                queue.push(alt, adjacency.node_index.0);
                dist_to[adjacency.node_index.0] = alt;
                parent[adjacency.node_index.0] = Some(v);
            }
        }
    }
    (dist_to, parent)
}
