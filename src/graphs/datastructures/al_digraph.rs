use crate::graphs::datastructures::digraph::{Digraph, DigraphAdjacency, NodeData};
use std::fmt;

#[derive(Clone, Debug)]
struct ALDigraphEdge {
    to: usize,
    weight: f64,
}

#[derive(Debug)]
pub struct ALDigraph {
    num_vertices: usize,
    adjacency_lists: Vec<Vec<ALDigraphEdge>>,
}

impl fmt::Display for ALDigraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: `f`
        // Returns `fmt::Result` which indicates whether the operation succeeded or failed
        writeln!(f, "{} nodes", self.num_vertices)
        // Replace the above with the below for full output
        //        writeln!(f, "{} nodes", self.num_vertices)?;
        //         self.adjacency_lists.iter().enumerate().fold(
        //             Ok(()),
        //             |result, (from_node, adjacency_list)| {
        //                 result.and_then(|_| {
        //                     writeln!(f, "\t{}", from_node)?;
        //                     adjacency_list.iter().fold(Ok(()), |result, edge| {
        //                         result.and_then(|_| writeln!(f, "\t\t{}", edge))
        //                     })
        //                 })
        //             },
        //         )
    }
}

impl ALDigraph {
    pub fn new(num_vertices: usize) -> Self {
        Self {
            num_vertices,
            adjacency_lists: vec![Vec::new(); num_vertices],
        }
    }
}

impl Digraph for ALDigraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn add_node_data(&mut self, node_id: usize, longitude: f64, latitude: f64) {}

    fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        let e = ALDigraphEdge { to, weight };
        self.adjacency_lists[from].push(e);
    }

    fn adj(&self, node_number: usize) -> Vec<DigraphAdjacency> {
        self.adjacency_lists[node_number]
            .iter()
            .map(|edge| DigraphAdjacency {
                node_index: edge.to,
                node_longitude: 0.0,
                node_latitude: 0.0,
                weight: edge.weight,
            })
            .collect()
    }

    fn dist(&self, from_node: usize, to_node: usize) -> f64 {
        for u in self.adj(from_node) {
            if u.node_index == to_node {
                return u.weight;
            }
        }
        panic!("Node not connected!")
    }

    fn get_node_data(&self, node_id: usize) -> NodeData {
        NodeData {
            node_index: node_id,
            longitude: 0.0,
            latitude: 0.0,
        }
    }
}
