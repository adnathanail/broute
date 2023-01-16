use crate::graphs::datastructures::digraph::{Digraph, DigraphAdjacency, NodeData, NodeID, NodeIndex, NodesData};
use std::collections::HashMap;
use std::fmt;

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

impl AMDigraph {
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

    fn add_edge(&mut self, from_id: NodeID, to_id: NodeID, weight: f64) {
        self.distance_matrix[self.nodes_data.get_node_index_by_id(&from_id).0]
            [self.nodes_data.get_node_index_by_id(&to_id).0] = weight;
    }

    fn adj(&self, node_index: NodeIndex) -> Vec<DigraphAdjacency> {
        self.distance_matrix[node_index.0]
            .iter()
            .enumerate()
            .map(|(to, weight)| DigraphAdjacency {
                node_index: NodeIndex(to),
                weight: *weight,
            })
            .collect()
    }

    fn dist(&self, from_index: NodeIndex, to_index: NodeIndex) -> f64 {
        self.distance_matrix[from_index.0][to_index.0]
    }

    fn nodes_data(&self) -> &NodesData {
        &self.nodes_data
    }

    fn mut_nodes_data(&mut self) -> &mut NodesData {
        & mut self.nodes_data
    }
}
