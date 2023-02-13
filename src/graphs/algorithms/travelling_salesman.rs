use crate::graphs::datastructures::{AMDigraph, Digraph, GraphPath, NodeID, NodeIndex};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use crate::graphs::algorithms::Dijkstra;

pub fn form_abstracted_graph(g: &dyn Digraph, node_ids: &Vec<NodeID>) -> AMDigraph {
    let mut abstracted_graph = AMDigraph::new(node_ids.len());
    for node_id in node_ids {
        let node_data = g.nodes_data().get_node_data_by_id(*node_id);
        abstracted_graph
            .mut_nodes_data()
            .add_node_data(*node_id, *node_data)
    }
    for from_node_id in node_ids {
        let from_node_index = g.nodes_data().get_node_index_by_id(from_node_id);
        let mut dj = Dijkstra::new(g, *from_node_index);
        dj.run();
        for to_node_id in node_ids {
            if to_node_id != from_node_id {
                let to_node_index = g.nodes_data().get_node_index_by_id(to_node_id);
                abstracted_graph.add_edge_by_id(
                    *from_node_id,
                    *to_node_id,
                    dj.get_dist_to(*to_node_index),
                )
            }
        }
    }
    abstracted_graph
}

fn get_potential_new_path(
    rng: &mut ThreadRng,
    g: &dyn Digraph,
    current_path: &GraphPath,
) -> GraphPath {
    let mut potential_new_path = current_path.clone();

    let node_index_to_mutate = rng.gen_range(0..(g.num_vertices() - 1));

    let reverse_or_transport: bool = rng.gen();

    if reverse_or_transport {
        let node_index_to_swap_with = if node_index_to_mutate < (g.num_vertices() - 1) {
            node_index_to_mutate + 1
        } else {
            0
        };
        potential_new_path
            .path
            .swap(node_index_to_mutate, node_index_to_swap_with);
    } else {
        // Cyclic permutation
        let node_to_move = potential_new_path.path[node_index_to_mutate];
        // -2 because we are looking for new position with 1 node missing
        let new_node_position = rng.gen_range(0..(g.num_vertices() - 2));
        potential_new_path.path.remove(node_index_to_mutate);
        potential_new_path
            .path
            .insert(new_node_position, node_to_move)
    }

    potential_new_path
}

pub fn travelling_salesman(g: &dyn Digraph, output_graph: bool) -> GraphPath {
    let mut result_data: Vec<(f64, f64)> = vec![];

    let mut rng = thread_rng();

    let mut current_path = GraphPath {
        path: (0..g.num_vertices()).map(NodeIndex).collect(),
    };
    current_path.path.shuffle(&mut rng);
    let mut path_length = current_path.get_length_on_graph(g);

    let mut best_path = current_path.clone();

    // println!("Initial state");
    //
    // println!("\t{:?}", best_path.path);
    // println!("\t{}", path_length);

    let mut temp = f64::sqrt(g.num_vertices() as f64);
    let mut iterations = 0;
    while temp > 1e-8_f64 && iterations < (100 * g.num_vertices()) {
        // println!("{}", temp);
        let potential_new_path = get_potential_new_path(&mut rng, g, &current_path);

        let new_path_length = potential_new_path.get_length_on_graph(g);
        if new_path_length < path_length {
            current_path = potential_new_path;
            best_path.clone_from(&current_path);
            path_length = new_path_length;
        } else {
            // TODO: Is this between 0 and 1?
            if f64::exp(-f64::abs(new_path_length - path_length) / temp) > rng.gen::<f64>() {
                current_path = potential_new_path;
                path_length = new_path_length;
            }
        }

        temp *= 0.995;
        iterations += 1;
        result_data.push((temp, path_length));
    }

    if output_graph {
        // We create our scatter plot from the data
        let s1: Plot =
            Plot::new(result_data.clone()).line_style(LineStyle::new().colour("#DD3355"));

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)
            .x_label("Temperature")
            .y_label("Path length")
            .y_range(0.0, result_data[0].1 + 100.0);

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save("out/tsp_test_1.svg").unwrap();
    }

    best_path
}
