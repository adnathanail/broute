use crate::graphs::datastructures::digraph::{Digraph, DigraphAdjacency, NodeIndex, NodesData};
use std::fmt;

/// Struct representating a graph as an adjacency matrix
#[derive(Debug)]
pub struct AMDigraph {
    // Because this struct has at least one private field, whilst it itself is pub(lic), it cannot
    //   be initialised by anything outside of this module
    // The only way to create a Graph object, is using the constructor defined below
    num_vertices: usize,
    distance_matrix: Vec<Vec<f64>>,
    nodes_data: NodesData,
}

impl fmt::Display for AMDigraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} nodes", self.num_vertices)
    }
}

impl AMDigraph {
    /// Create a new `AMDigraph` with a given number of nodes
    pub fn new(num_vertices: usize) -> Self {
        Self {
            num_vertices,
            distance_matrix: vec![vec![f64::MAX; num_vertices]; num_vertices],
            nodes_data: NodesData::new(),
        }
    }
}

impl Digraph for AMDigraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn add_edge_by_index(&mut self, from_index: NodeIndex, to_index: NodeIndex, weight: f64) {
        self.distance_matrix[from_index.0][to_index.0] = weight;
    }

    fn adj(&self, node_index: NodeIndex) -> Vec<DigraphAdjacency> {
        self.distance_matrix[node_index.0]
            .iter()
            .enumerate()
            .map(|(to, weight)| DigraphAdjacency {
                node_index: NodeIndex(to),
                weight: *weight,
            })
            .filter(|adjacency| adjacency.weight != f64::MAX)
            .collect()
    }

    fn dist(&self, from_index: NodeIndex, to_index: NodeIndex) -> f64 {
        self.distance_matrix[from_index.0][to_index.0]
    }

    fn nodes_data(&self) -> &NodesData {
        &self.nodes_data
    }

    fn mut_nodes_data(&mut self) -> &mut NodesData {
        &mut self.nodes_data
    }
}
