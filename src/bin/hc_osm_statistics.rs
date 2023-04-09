use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::{form_abstracted_graph, tsp_with_repeats, ConnectedComponents};
use broute::graphs::datastructures::{Digraph, NodeID};
use broute::graphs::input::load_pbf_file;
use broute::utils::{get_random_lat_lngs, mean, std_deviation};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::time::SystemTime;

fn run_full_tsp<T: Digraph>(g: &T, lat_lng_list: Vec<LatLng>) {
    let node_id_list = g
        .nodes_data()
        .get_node_ids_closest_to_lat_lngs(lat_lng_list);

    let abstracted_graph = form_abstracted_graph(g, &node_id_list);

    let best_path = tsp_with_repeats(&abstracted_graph, 5);

    let p_node_ids: Vec<NodeID> = best_path
        .path
        .iter()
        .map(|node_index| {
            *abstracted_graph
                .nodes_data()
                .get_node_id_by_index(node_index)
        })
        .collect();
}

fn main() {
    let g = load_pbf_file("test_data/osm/central-london-latest.osm.pbf").unwrap();

    println!("Original graph {:} nodes", g.num_vertices());

    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();

    println!("Biggest connected subgraph {:} nodes", c_g.num_vertices());

    let mut rng = Pcg64Mcg::from_entropy();

    println!("graph_num_nodes,duration_mean,duration_std_deviation");
    for num_nodes in [5, 10, 25, 50, 100, 250, 500, 1000] {
        let mut durations: Vec<f64> = vec![];

        const NUM_ITERATIONS: i32 = 5;

        for i in 0..NUM_ITERATIONS {
            println!("{i}");
            let lat_lng_list = get_random_lat_lngs(num_nodes, &mut rng);

            let start = SystemTime::now();

            run_full_tsp(&c_g, lat_lng_list);

            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();

            durations.push((duration.as_millis() as f64) / 1000.0);
        }
        println!(
            "{},{},{}",
            num_nodes,
            mean(&durations),
            std_deviation(&durations),
        );
    }
}
