use crate::graphs::datastructures::digraph::Digraph;
use std::fmt;

#[derive(Debug)]
pub struct AMDigraph {
    // Because this struct has at least one private field, whilst it itself is pub(lic), it cannot
    //   be initialised by anything outside of this module
    // The only way to create a Graph object, is using the constructor defined below
    num_vertices: usize,
    distance_matrix: Vec<Vec<f64>>,
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
        }
    }
}

impl Digraph for AMDigraph {
    fn num_vertices(&self) -> usize {
        return self.num_vertices;
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.distance_matrix[from][to] = weight;
    }

    fn adj(&self, node_number: usize) -> &Vec<f64> {
        &self.distance_matrix[node_number]
    }

    fn dist(&self, from_node: usize, to_node: usize) -> f64 {
        self.distance_matrix[from_node][to_node]
    }
}
