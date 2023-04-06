use std::cmp::max;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::LineStyle;
// use plotlib::view::ContinuousView;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

use crate::graphs::algorithms::AStar;
use crate::graphs::datastructures::{AMDigraph, Digraph, GraphPath, NodeID, NodeIndex};

/// Given a graph and a subset of the `NodeID`s of that graph,
/// produce a new graph containing just those nodes,
/// with all distances between the nodes calculated with A*
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

fn two_opt_cost<T: Digraph>(g: &T, p: &GraphPath, first: usize, second: usize) -> f64 {
    let a = p.path[first];
    let b = p.path[(first + 1) % p.path.len()];
    let c = p.path[second];
    let d = p.path[(second + 1) % p.path.len()];
    let mut length_delta = -g.dist(a, b) + g.dist(a, c);
    if second < p.path.len() - 1 {
        length_delta += -g.dist(c, d) + g.dist(b, d)
    }
    length_delta
}

fn two_opt(p: &GraphPath, first: usize, second: usize) -> GraphPath {
    let mut new_path = p.clone();
    new_path.path[(first + 1)..(second + 1)].reverse();
    new_path
}

/// Run the TSP solver multiple times, taking the best result
pub fn tsp_with_repeats<T: Digraph>(g: &T, repeats: usize) -> GraphPath {
    let mut sa = HillClimbing::new(g);
    sa.run();
    let mut best_path = sa.get_best_path().clone();
    for _ in 0..repeats {
        sa.run();
        if sa.get_best_path().get_length_on_graph(g) < best_path.get_length_on_graph(g) {
            best_path = sa.get_best_path().clone();
        }
    }
    best_path
}

/// Hill climbing based Travelling Salesman Problem solver
/// ```rust
/// use broute::graphs::algorithms::HillClimbing;
/// use broute::graphs::input::load_tsplib_file;
/// let g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();
/// let mut hc = HillClimbing::new(&g);
/// hc.run();
/// println!("Best path length: {}", hc.get_best_path().get_length_on_graph(&g));
/// ```
pub struct HillClimbing<'a, T: Digraph> {
    g: &'a T,
    num_iterations: usize,
    // result_data: Vec<(f64, f64)>,
    best_path: GraphPath,
    // path_length: f64,
    rng: Pcg64Mcg,
}

impl<'a, T: Digraph> HillClimbing<'a, T> {
    /// Create a new instance of the solver
    pub fn new(g: &'a T) -> Self {
        Self::new_with_custom_parameters(g, max(g.num_vertices().pow(2), 100))
    }

    /// Create a new instance of the solver
    pub fn new_with_custom_parameters(g: &'a T, num_iterations: usize) -> Self {
        let mut rng = Pcg64Mcg::from_entropy();

        let mut current_path = GraphPath {
            path: (0..g.num_vertices()).map(NodeIndex).collect(),
        };
        current_path.path.shuffle(&mut rng);

        // let path_length = current_path.get_length_on_graph(g);

        HillClimbing {
            g,
            num_iterations,
            // result_data: vec![],
            best_path: current_path,
            // path_length,
            rng,
        }
    }

    /// Run the solver
    pub fn run(&mut self) {
        // No meaningful permutations for 0, 1, 2 nodes
        if self.best_path.path.len() < 3 {
            return;
        }

        for _ in 0..self.num_iterations {
            let mut i = self.rng.gen_range(1..self.best_path.path.len() - 1);
            let mut j = self.rng.gen_range(1..self.best_path.path.len() - 1);
            if i > j {
                (i, j) = (j, i);
            }
            let length_delta = two_opt_cost::<T>(self.g, &self.best_path, i, j);
            if length_delta < 0.0 {
                // self.path_length += length_delta;
                self.best_path = two_opt(&self.best_path, i, j);
            }
            // self.result_data.push((i as f64, self.path_length));
        }
    }

    /// Get the best path found by the algorithm (only valid after running the algorithm)
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

#[cfg(test)]
mod tests {
    use crate::graphs::algorithms::travelling_salesman::{two_opt, two_opt_cost};
    use crate::graphs::datastructures::{GraphPath, NodeIndex};
    use crate::graphs::input::load_tsplib_file;
    use rand_distr::num_traits::abs;
    use std::cmp::{max, min};

    #[test]
    fn two_opt_test() {
        let g = load_tsplib_file("test_data/dimacs_tsp/test.tsp", usize::MAX).unwrap();
        let path = GraphPath {
            path: vec![
                NodeIndex(0),
                NodeIndex(1),
                NodeIndex(2),
                NodeIndex(3),
                NodeIndex(4),
                NodeIndex(5),
                NodeIndex(6),
                NodeIndex(7),
                NodeIndex(8),
                NodeIndex(9),
            ],
        };
        for i in 0..10 {
            for j in 0..10 {
                if i != j {
                    let (first, second) = (min(i, j), max(i, j));
                    let new_path = two_opt(&path, first, second);
                    let new_path_length = two_opt_cost(&g, &path, first, second);
                    let actual_new_path_length =
                        new_path.get_length_on_graph(&g) - path.get_length_on_graph(&g);
                    // Close enough
                    assert!(abs(new_path_length - actual_new_path_length) < 0.0000000000002);
                }
            }
        }
    }
}
