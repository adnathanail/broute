use std::cmp::min;
use std::collections::HashMap;
use crate::graphs::datastructures::al_digraph::ALDigraph;
use crate::graphs::datastructures::digraph::{Digraph, NodeID, NodeIndex};

#[cfg(test)]
#[path = "connected_components_tests.rs"]
mod connected_components_tests;

pub struct ConnectedComponents<'a> {
    g: &'a dyn Digraph,
    index: i32,
    node_stack: Vec<usize>,
    indexes: Vec<i32>,
    low_links: Vec<i32>,
    components: Vec<Vec<NodeIndex>>,
}

impl<'a> ConnectedComponents<'a> {
    /// Implements Tarjan's strongly connected component algorithm
    ///
    /// Original code based on pseudocode here
    ///   https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm#The_algorithm_in_pseudocode
    pub fn new(g: &'a dyn Digraph) -> Self {
        ConnectedComponents {
            g,
            index: 0,
            node_stack: vec![],
            indexes: vec![-1; g.num_vertices()],
            low_links: vec![-1; g.num_vertices()],
            components: vec![],
        }
    }

    pub fn run(&mut self) {
        for i in 0..self.g.num_vertices() {
            if self.indexes[i] == -1 {
                self.strong_connect(i);
            }
        }
    }

    fn strong_connect(&mut self, v: usize) {
        self.indexes[v] = self.index;
        self.low_links[v] = self.index;
        self.index += 1;
        self.node_stack.push(v);

        for w in self.g.adj(NodeIndex(v)) {
            if self.indexes[(&w).node_index.0] == -1 {
                self.strong_connect((&w).node_index.0);
                self.low_links[v] = min(self.low_links[v], self.low_links[(&w).node_index.0]);
            } else if self.node_stack.contains(&(&w).node_index.0) {
                self.low_links[v] = min(self.low_links[v], self.indexes[(&w).node_index.0])
            }
        }

        if self.low_links[v] == self.indexes[v] {
            let mut w: i32 = -1;
            let mut node_indexes_this_component: Vec<NodeIndex> = vec![];
            while w != v as i32 {
                w = self.node_stack.pop().unwrap() as i32;
                node_indexes_this_component.push(NodeIndex(w as usize))
            }
            self.components.push(node_indexes_this_component);
        }
    }

    pub fn get_connected_subgraphs(self, min_graph_size: usize) -> Vec<ALDigraph> {
        let mut out = vec![];
        for component in self.components {
            if (&component).len() < min_graph_size {
                continue
            }

            let mut g = ALDigraph::new((&component).len());
            for u in &component {
                let u_id = self.g.nodes_data().get_node_id_by_index(u);
                g.mut_nodes_data().add_node_data(
                    *u_id,
                    *self.g.nodes_data().get_node_data_by_index(*u)
                );
            }
            for u in &component {
                let u_id = self.g.nodes_data().get_node_id_by_index(&u);
                for v in self.g.adj(*u) {
                    let v_id = self.g.nodes_data().get_node_id_by_index(&NodeIndex(v.node_index.0));
                    if (&component).contains(&v.node_index) {
                        g.add_edge_by_id(*u_id, *v_id, v.weight)
                    }
                }
            }
            out.push(g)
        }
        out
    }

    pub fn get_largest_connected_subgraphs(self) -> ALDigraph {
        let mut largest_graph_size = 0;
        let mut largest_graph: Option<ALDigraph> = None;
        for g in self.get_connected_subgraphs(2) {
            if g.num_vertices() > largest_graph_size {
                largest_graph_size = g.num_vertices();
                largest_graph = Some(g);
            }
        }
        largest_graph.unwrap()
    }
}
