use crate::graphs::datastructures::digraph::{Digraph, DigraphAdjacency, NodeIndex, NodesData};
use std::fmt;

#[derive(Clone, Debug)]
struct ALDigraphEdge {
    to: NodeIndex,
    weight: f64,
}

#[derive(Debug)]
pub struct ALDigraph {
    num_vertices: usize,
    adjacency_lists: Vec<Vec<ALDigraphEdge>>,
    nodes_data: NodesData,
}

impl fmt::Display for ALDigraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} nodes", self.num_vertices)
    }
}

impl ALDigraph {
    pub fn new(num_vertices: usize) -> Self {
        Self {
            num_vertices,
            adjacency_lists: vec![Vec::new(); num_vertices],
            nodes_data: NodesData::new(),
        }
    }
}

impl Digraph for ALDigraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn add_edge_by_index(&mut self, from_index: NodeIndex, to_index: NodeIndex, weight: f64) {
        let e = ALDigraphEdge {
            to: to_index,
            weight,
        };
        self.adjacency_lists[from_index.0].push(e);
    }

    fn adj(&self, node_index: NodeIndex) -> Vec<DigraphAdjacency> {
        self.adjacency_lists[node_index.0]
            .iter()
            .map(|edge| DigraphAdjacency {
                node_index: edge.to,
                weight: edge.weight,
            })
            .collect()
    }

    fn dist(&self, from_index: NodeIndex, to_index: NodeIndex) -> f64 {
        for u in self.adj(from_index) {
            if u.node_index == to_index {
                return u.weight;
            }
        }
        panic!("Node not connected!")
    }

    fn nodes_data(&self) -> &NodesData {
        &self.nodes_data
    }

    fn mut_nodes_data(&mut self) -> &mut NodesData {
        &mut self.nodes_data
    }
}
