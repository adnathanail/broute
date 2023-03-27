use crate::graphs::algorithms::AStar;
use crate::graphs::datastructures::{AMDigraph, Digraph, GraphPath, NodeID, NodeIndex};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use std::f64::consts::E;

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

pub struct SimulatedAnnealing<'a, T: Digraph> {
    g: &'a T,
    initial_temperature: f64,
    iterations_per_temperature: usize,
    cooling_rate: f64,
    minimum_temperature: f64,
    result_data: Vec<(f64, f64)>,
    current_path: GraphPath,
    path_length: f64,
    best_path: GraphPath,
    rng: ThreadRng,
}

impl<'a, T: Digraph> SimulatedAnnealing<'a, T> {
    pub fn new(g: &'a T) -> Self {
        Self::new_with_custom_parameters(g, 100.0, 1, 0.999, 1e-9_f64)
    }

    pub fn new_with_custom_parameters(
        g: &'a T,
        initial_temperature: f64,
        iterations_per_temperature: usize,
        cooling_rate: f64,
        minimum_temperature: f64,
    ) -> Self {
        let mut rng = thread_rng();

        let mut current_path = GraphPath {
            path: (0..g.num_vertices()).map(NodeIndex).collect(),
        };
        current_path.path.shuffle(&mut rng);

        let path_length = current_path.get_length_on_graph(g);

        let best_path = current_path.clone();
        SimulatedAnnealing {
            g,
            initial_temperature,
            iterations_per_temperature,
            cooling_rate,
            minimum_temperature,
            result_data: vec![],
            current_path,
            path_length,
            best_path,
            rng,
        }
    }

    pub fn run(&mut self) {
        // No meaningful permutations for 0, 1, 2 nodes
        if self.current_path.path.len() < 3 {
            return;
        }

        let mut temp = self.initial_temperature;
        while temp > self.minimum_temperature {
            for _ in 0..self.iterations_per_temperature {
                let new_path = self.get_potential_new_path();
                let new_path_length = new_path.get_length_on_graph(self.g);
                let delta_cost = new_path_length - self.path_length;

                if delta_cost < 0.0 || self.rng.gen::<f64>() < E.powf(-delta_cost / temp) {
                    self.current_path = new_path;
                    self.path_length = new_path_length;
                }
                if delta_cost < 0.0 {
                    self.best_path.clone_from(&self.current_path);
                }
            }

            temp *= self.cooling_rate;
            self.result_data.push((temp, self.path_length));
        }
    }

    fn get_potential_new_path(&mut self) -> GraphPath {
        let mut new_path = self.current_path.clone();

        let i = self.rng.gen_range(1..new_path.path.len() - 1);
        let j = self.rng.gen_range(1..new_path.path.len() - 1);
        new_path.path[min(i, j)..max(i, j)].reverse();

        new_path
    }

    pub fn get_best_path(&self) -> &GraphPath {
        &self.best_path
    }

    pub fn output_graph(&self) {
        // We create our scatter plot from the data
        let s1: Plot =
            Plot::new(self.result_data.clone()).line_style(LineStyle::new().colour("#DD3355"));

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)
            .x_label("Temperature")
            .y_label("Path length")
            .y_range(0.0, self.result_data[0].1 + 100.0);

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save("out/tsp_test_1.svg").unwrap();
    }
}
