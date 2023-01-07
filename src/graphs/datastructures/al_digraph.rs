use crate::graphs::datastructures::digraph::{Digraph, DigraphAdjacency, NodeData};
use std::collections::HashMap;
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
    node_data: HashMap<usize, NodeData>,
    node_id_index_lookup: HashMap<usize, usize>,
    current_node_index: usize,
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
            node_data: HashMap::new(),
            node_id_index_lookup: HashMap::new(),
            current_node_index: 0,
        }
    }
}

impl Digraph for ALDigraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn add_node_data(&mut self, node_id: usize, longitude: f64, latitude: f64) {
        self.node_id_index_lookup
            .insert(node_id, self.current_node_index);
        self.current_node_index += 1;
        self.node_data.insert(
            self.node_id_index_lookup[&node_id],
            NodeData {
                longitude,
                latitude,
            },
        );
    }

    fn add_edge(&mut self, from_id: usize, to_id: usize, weight: f64) {
        let e = ALDigraphEdge {
            to: *self.node_id_index_lookup.get(&to_id).unwrap(),
            weight,
        };
        self.adjacency_lists[*self.node_id_index_lookup.get(&from_id).unwrap()].push(e);
    }

    fn get_node_data(&self, node_id: usize) -> &NodeData {
        self.node_data.get(&node_id).unwrap()
    }

    fn adj(&self, node_index: usize) -> Vec<DigraphAdjacency> {
        self.adjacency_lists[node_index]
            .iter()
            .map(|edge| DigraphAdjacency {
                node_index: edge.to,
                weight: edge.weight,
            })
            .collect()
    }

    fn dist(&self, from_index: usize, to_index: usize) -> f64 {
        for u in self.adj(from_index) {
            if u.node_index == to_index {
                return u.weight;
            }
        }
        panic!("Node not connected!")
    }
}
