use std::time::Instant;
use crate::graphs::algorithms::Dijkstra;
use crate::graphs::datastructures::{AMDigraph, Digraph, GraphPath, NodeID, NodeIndex};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::view::ContinuousView;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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

pub fn generate_path(rng: &mut ThreadRng, path_length: usize) -> GraphPath {
    let mut path = GraphPath {
        path: (0..path_length).map(|n| NodeIndex(n)).collect(),
    };
    path.path.shuffle(rng);
    path
}

fn get_potential_new_path(rng: &mut ThreadRng, g: &dyn Digraph, current_path: &GraphPath) -> GraphPath {
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

pub fn travelling_salesman(g: &dyn Digraph, desired_duration_millis: f64) -> GraphPath {
    let mut result_data: Vec<(f64, f64)> = vec![];

    let mut rng = thread_rng();

    let mut best_path = generate_path(&mut rng, g.num_vertices());

    let start_time = Instant::now();

    let mut iters = 0;

    loop {
        let portion_elapsed = (start_time.elapsed().as_millis() as f64) / desired_duration_millis;

        if portion_elapsed >= 1.0 {
            break;
        }

        let potential_new_path = get_potential_new_path(&mut rng, g, &best_path);

        if get_path_length(g, &potential_new_path) < get_path_length(g, &best_path) {
            best_path = potential_new_path;
        } else {
            if 1.0f64.exp().powf(-10.0 * portion_elapsed.powf(3.0)) > rng.gen::<f64>() {
                best_path = potential_new_path;
            }
        }

        iters += 1;
        println!("{}", get_path_length(g, &best_path));
        result_data.push((portion_elapsed, get_path_length(g, &best_path)));
    }

    println!("{} iters", iters);

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

// 1060 iters
// pub fn old_get_path_length(g: &DigraphAM, path: &GraphPath) -> f64 {
//    (0..(path.path.len() - 1)).fold(0f64, |total, i| {
//        total + g.dist(path.path[i], path.path[i + 1])
//    })
//}

// 1713 iters
pub fn get_path_length(g: &dyn Digraph, path: &GraphPath) -> f64 {
    let mut route_iter = path.path.iter();
    let mut current_city = match route_iter.next() {
        None => return 0.0,
        Some(v) => *v,
    };

    route_iter.fold(0.0, |mut total_distance, &next_city| {
        total_distance += g.dist(current_city, next_city);
        current_city = next_city;
        total_distance
    })
}
