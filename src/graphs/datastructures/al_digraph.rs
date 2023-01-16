use crate::graphs::datastructures::digraph::{Digraph, DigraphAdjacency, NodeData, NodeID, NodeIndex, NodesData};
use std::collections::HashMap;
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
            nodes_data: NodesData::new(),
        }
    }
}

impl Digraph for ALDigraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn add_edge(&mut self, from_id: NodeID, to_id: NodeID, weight: f64) {
        let e = ALDigraphEdge {
            to: *self.nodes_data.get_node_index_by_id(&to_id),
            weight,
        };
        self.adjacency_lists[self.nodes_data.get_node_index_by_id(&from_id).0].push(e);
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
        & mut self.nodes_data
    }
}
