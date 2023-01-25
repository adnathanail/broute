use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};

fn cc_dfs(g: &dyn Digraph, marked: &mut Vec<bool>, cc: &mut Vec<i32>, count: i32, v: usize) {
    marked[v] = true;
    cc[v] = count;
    for w in g.adj(NodeIndex(v)) {
        if !marked[w.node_index.0] {
            cc_dfs(g, marked, cc, count, w.node_index.0);
        }
    }
}

pub fn connected_components(g: &dyn Digraph) {
    let mut marked = vec![false; g.num_vertices()];
    let mut cc = vec![-1; g.num_vertices()];
    let mut count = 0;
    for v in 0..g.num_vertices() {
        if !marked[v] {
            cc_dfs(g, &mut marked, &mut cc, count, v);
            count += 1;
        }
    }
    for component in 0..count {
        let mut num_this_component = 0;
        for i in 0..g.num_vertices() {
            if cc[i] == component {
                num_this_component += 1;
            }
        }
        println!("Component {:} Count {:}", component, num_this_component)
    }
    println!("{:}", count);
    for i in 0..g.num_vertices() {
        println!("ID {:} Component {:}", i, cc[i])
    }
}
