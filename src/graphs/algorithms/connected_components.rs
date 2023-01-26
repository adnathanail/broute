use std::cmp::min;
use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};

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

        for component in 0..self.count {
            let mut num_this_component = 0;
            for i in 0..self.g.num_vertices() {
                if self.cc[i] == component {
                    num_this_component += 1;
                }
            }
            println!("Component {:} Count {:}", component, num_this_component)
        }
        println!("{:}", self.count);
        for i in 0..self.g.num_vertices() {
            println!("ID {:} Component {:}", i, self.cc[i])
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
}
