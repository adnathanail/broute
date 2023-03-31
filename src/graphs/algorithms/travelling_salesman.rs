use std::cmp::{max, min};

// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::LineStyle;
// use plotlib::view::ContinuousView;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::graphs::algorithms::AStar;
use crate::graphs::datastructures::{AMDigraph, Digraph, GraphPath, NodeID, NodeIndex};

pub fn form_abstracted_graph(g: &impl Digraph, node_ids: &Vec<NodeID>) -> AMDigraph {
    let node_indexes: Vec<NodeIndex> = node_ids
        .iter()
        .map(|node_id| *g.nodes_data().get_node_index_by_id(node_id))
        .collect();
    let mut abstracted_graph = AMDigraph::new(node_ids.len());
    for node_id in node_ids {
        let node_data = g.nodes_data().get_node_data_by_id(*node_id);
        abstracted_graph
            .mut_nodes_data()
            .add_node_data(*node_id, *node_data)
    }
    for from_node_id in node_ids {
        let from_node_index = g.nodes_data().get_node_index_by_id(from_node_id);
        let mut astar = AStar::new(g, *from_node_index, node_indexes.clone());
        astar.run();
        for to_node_id in node_ids {
            if to_node_id != from_node_id {
                let to_node_index = g.nodes_data().get_node_index_by_id(to_node_id);
                abstracted_graph.add_edge_by_id(
                    *from_node_id,
                    *to_node_id,
                    astar.get_dist_to_to_node(*to_node_index).unwrap(),
                )
            }
        }
    }
    abstracted_graph
}

pub fn two_opt(p: &GraphPath, v1: NodeIndex, v2: NodeIndex) -> GraphPath {
    let mut new_path = p.path.clone();
    let mut i = v1.0;
    let mut j = v2.0;
    while i != j {
        new_path.swap(i, j);
        i = (i + 1) % new_path.len();
        if i == j {
            break;
        }
        j = if j == 0 { new_path.len() - 1 } else { j - 1 };
    }
    GraphPath { path: new_path }
}

pub struct HillClimbing<'a, T: Digraph> {
    g: &'a T,
    num_iterations: usize,
    // result_data: Vec<(f64, f64)>,
    best_path: GraphPath,
    path_length: f64,
    rng: ThreadRng,
}

impl<'a, T: Digraph> HillClimbing<'a, T> {
    pub fn new(g: &'a T) -> Self {
        Self::new_with_custom_parameters(g, g.num_vertices().pow(2))
    }

    pub fn new_with_custom_parameters(g: &'a T, num_iterations: usize) -> Self {
        let mut rng = thread_rng();

        let mut current_path = GraphPath {
            path: (0..g.num_vertices()).map(NodeIndex).collect(),
        };
        current_path.path.shuffle(&mut rng);

        let path_length = current_path.get_length_on_graph(g);

        HillClimbing {
            g,
            num_iterations,
            // result_data: vec![],
            best_path: current_path,
            path_length,
            rng,
        }
    }

    pub fn run(&mut self) {
        // No meaningful permutations for 0, 1, 2 nodes
        if self.best_path.path.len() < 3 {
            return;
        }

        for _i in 0..self.num_iterations {
            let new_path = self.get_potential_new_path();
            let new_path_length = new_path.get_length_on_graph(self.g);

            if new_path_length < self.path_length {
                self.best_path = new_path;
                self.path_length = new_path_length;
            }
            // self.result_data.push((i as f64, self.path_length));
        }
        //     TODO realign best_path so that the longest edge is removed
    }

    fn get_potential_new_path(&mut self) -> GraphPath {
        let i = self.rng.gen_range(1..self.best_path.path.len() - 1);
        let j = self.rng.gen_range(1..self.best_path.path.len() - 1);

        two_opt(&self.best_path, NodeIndex(i), NodeIndex(j))
    }

    pub fn get_best_path(&self) -> &GraphPath {
        &self.best_path
    }

    // pub fn output_graph(&self, path: &str) {
    //     let s1: Plot =
    //         Plot::new(self.result_data.clone()).line_style(LineStyle::new().colour("#DD3355"));
    //
    //     let v = ContinuousView::new()
    //         .add(s1)
    //         .x_label("Iterations")
    //         .y_label("Path length")
    //         .y_range(0.0, self.result_data[0].1 + 100.0);
    //
    //     Page::single(&v).save(path).unwrap();
    // }
}
