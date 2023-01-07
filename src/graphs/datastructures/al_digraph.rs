use std::collections::HashMap;
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
    current_node_index: usize,
    node_data: HashMap<usize, NodeData>,
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
        let mut out = Self {
            num_vertices,
            adjacency_lists: vec![Vec::new(); num_vertices],
            current_node_index: 0,
            node_data: HashMap::new(),
        };
        out
    }
}

impl Digraph for ALDigraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn add_node_data(&mut self, node_id: usize, longitude: f64, latitude: f64) {
        self.node_data.insert(node_id, NodeData { node_index: self.current_node_index, longitude, latitude });
        self.current_node_index += 1;
    }

    fn add_edge(&mut self, from_id: usize, to_id: usize, weight: f64) {
        let e = ALDigraphEdge { to: self.node_data.get(&to_id).unwrap().node_index, weight };
        self.adjacency_lists[self.node_data.get(&from_id).unwrap().node_index].push(e);
    }

    fn adj(&self, node_id: usize) -> Vec<DigraphAdjacency> {
        self.adjacency_lists[self.node_data.get(&node_id).unwrap().node_index]
            .iter()
            .map(|edge| {
                let nd: &NodeData = self.node_data.get(&edge.to).unwrap();
                DigraphAdjacency {
                    node_data: NodeData {
                        node_index: nd.node_index,
                        longitude: nd.longitude,
                        latitude: nd.latitude,
                    },
                    weight: edge.weight,
                }
            })
            .collect()
    }

    fn dist(&self, from_id: usize, to_id: usize) -> f64 {
        for u in self.adj(self.node_data.get(&from_id).unwrap().node_index) {
            if u.node_data.node_index == self.node_data.get(&to_id).unwrap().node_index {
                return u.weight;
            }
        }
        panic!("Node not connected!")
    }

    fn get_node_data(&self, node_id: usize) -> &NodeData {
        self.node_data.get(&node_id).unwrap()
    }
}
