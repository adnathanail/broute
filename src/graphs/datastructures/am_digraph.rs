use crate::graphs::datastructures::digraph::{
    Digraph, DigraphAdjacency, NodeData, NodeID, NodeIndex,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct AMDigraph {
    // Because this struct has at least one private field, whilst it itself is pub(lic), it cannot
    //   be initialised by anything outside of this module
    // The only way to create a Graph object, is using the constructor defined below
    num_vertices: usize,
    distance_matrix: Vec<Vec<f64>>,
    node_data: HashMap<NodeIndex, NodeData>,
    node_id_index_lookup: HashMap<NodeID, NodeIndex>,
    current_node_index: NodeIndex,
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
            node_data: HashMap::new(),
            node_id_index_lookup: HashMap::new(),
            current_node_index: NodeIndex(0),
        }
    }
}

impl Digraph for AMDigraph {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn add_node_data(&mut self, node_id: NodeID, longitude: f64, latitude: f64) {
        self.node_id_index_lookup
            .insert(node_id, self.current_node_index);
        self.current_node_index.0 += 1;
        self.node_data.insert(
            self.node_id_index_lookup[&node_id],
            NodeData {
                longitude,
                latitude,
            },
        );
    }

    fn add_edge(&mut self, from_id: NodeID, to_id: NodeID, weight: f64) {
        self.distance_matrix[self.node_id_index_lookup.get(&from_id).unwrap().0]
            [self.node_id_index_lookup.get(&to_id).unwrap().0] = weight;
    }

    fn get_node_data(&self, node_id: NodeID) -> &NodeData {
        self.node_data
            .get(self.node_id_index_lookup.get(&node_id).unwrap())
            .unwrap()
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
}
