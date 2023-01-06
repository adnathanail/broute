use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use super::datastructures::digraph::Digraph;

#[derive(Debug, Clone)]
pub struct GraphPath {
    pub path: Vec<usize>,
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

pub fn travelling_salesman(g: &dyn Digraph) -> GraphPath {
    let mut result_data: Vec<(f64, f64)> = vec![];

    let mut rng = thread_rng();

    let mut best_path = GraphPath {
        path: (0..g.num_vertices()).collect(),
    };
    best_path.path.shuffle(&mut rng);
    let mut path_length = get_path_length(g, &best_path);

    println!("Initial state");

    println!("\t{:?}", best_path.path);
    println!("\t{}", path_length);

    let mut temp = f64::sqrt(g.num_vertices() as f64);
    let mut iterations = 0;
    while temp > 1e-8_f64 && iterations < (100 * g.num_vertices()) {
        println!("{}", temp);
        let potential_new_path = get_potential_new_path(&mut rng, g, &best_path);

        let new_path_length = get_path_length(g, &potential_new_path);
        if new_path_length < path_length {
            best_path = potential_new_path;
            path_length = new_path_length;
        } else {
            // TODO: Is this between 0 and 1?
            if f64::exp(-f64::abs(new_path_length - path_length) / temp) > rng.gen::<f64>() {
                best_path = potential_new_path;
                path_length = new_path_length;
            }
        }

        temp *= 0.995;
        iterations += 1;
        result_data.push((temp, path_length));
    }

    // We create our scatter plot from the data
    let s1: Plot = Plot::new(result_data.clone()).line_style(LineStyle::new().colour("#DD3355"));

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .x_label("Temperature")
        .y_label("Path length")
        .y_range(0.0, result_data[0].1 + 100.0);

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("out/temp_vs_cost.svg").unwrap();

    best_path
}

pub fn get_path_length(g: &dyn Digraph, path: &GraphPath) -> f64 {
    (0..(path.path.len() - 1)).fold(0f64, |total, i| {
        total + g.dist(path.path[i], path.path[i + 1])
    })
}
