use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};

pub struct ConnectedComponents<'a> {
    g: &'a dyn Digraph,
    marked: Vec<bool>,
    cc: Vec<i32>,
    count: i32,
}

impl<'a> ConnectedComponents<'a> {
    pub fn new(g: &'a dyn Digraph) -> Self {
        ConnectedComponents {
            g,
            marked: vec![false; g.num_vertices()],
            cc: vec![-1; g.num_vertices()],
            count: 0
        }
    }

    pub fn run(&mut self) {
        for v in 0..self.g.num_vertices() {
            if !self.marked[v] {
                self.cc_dfs(v);
                self.count += 1;
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

    fn cc_dfs(&mut self, v: usize) {
        self.marked[v] = true;
        self.cc[v] = self.count;
        for w in self.g.adj(NodeIndex(v)) {
            if !self.marked[w.node_index.0] {
                self.cc_dfs(w.node_index.0);
            }
        }
    }
}
