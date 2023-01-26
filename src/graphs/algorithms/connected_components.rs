use std::cmp::min;
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
    cc: Vec<i32>,
    count: i32,
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
            count: 0,
            cc: vec![-1; g.num_vertices()],
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
            while w != v as i32 {
                w = self.node_stack.pop().unwrap() as i32;
                self.cc[w as usize] = self.count;
            }
            self.count += 1;
        }
    }

    pub fn get_connected_subgraphs(self, min_graph_size: usize) -> Vec<ALDigraph> {
        let mut out = vec![];
        for component in 0..self.count {
            let mut num_this_component = 0;
            let mut node_indexes_this_component: Vec<NodeIndex> = vec![];
            for u in 0..self.g.num_vertices() {
                if self.cc[u] == component {
                    num_this_component += 1;
                    node_indexes_this_component.push(NodeIndex(u))
                }
            }
            if num_this_component < min_graph_size {
                continue
            }

            let mut g = ALDigraph::new(num_this_component);
            for u in &node_indexes_this_component {
                let u_id = self.g.nodes_data().get_node_id_by_index(&u);
                g.mut_nodes_data().add_node_data(
                    *u_id,
                    *self.g.nodes_data().get_node_data_by_index(*u)
                );
            }
            for u in &node_indexes_this_component {
                let u_id = self.g.nodes_data().get_node_id_by_index(&u);
                for v in self.g.adj(*u) {
                    let v_id = self.g.nodes_data().get_node_id_by_index(&NodeIndex(v.node_index.0));
                    if node_indexes_this_component.contains(&v.node_index) {
                        g.add_edge_by_id(*u_id, *v_id, v.weight)
                    }
                }
            }
            out.push(g)
        }
        out
    }
}
