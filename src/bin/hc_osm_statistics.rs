use broute::graphs::algorithms::{form_abstracted_graph, tsp_with_repeats, ConnectedComponents};
use broute::graphs::datastructures::{Digraph, NodeID};
use broute::graphs::input::load_pbf_file;
use broute::utils::{get_random_lat_lngs, mean, std_deviation};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use std::time::SystemTime;

fn main() {
    let g = load_pbf_file("test_data/osm/monaco-latest.osm.pbf").unwrap();

    println!("Original graph {:} nodes", g.num_vertices());

    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();

    println!("Biggest connected subgraph {:} nodes", c_g.num_vertices());

    let mut rng = Pcg64Mcg::from_entropy();

    println!("graph_num_nodes,geocoding_duration_mean,geocoding_duration_std_deviation,abstracted_graph_duration_mean,abstracted_graph_duration_std_deviation,tsp_duration_mean,tsp_duration_std_deviation");
    for num_nodes in [5, 10, 25, 50, 100, 250, 500, 1000] {
        let mut geocoding_durations: Vec<f64> = vec![];
        let mut abstracted_graph_durations: Vec<f64> = vec![];
        let mut tsp_durations: Vec<f64> = vec![];

        const NUM_ITERATIONS: i32 = 10;

        for _ in 0..NUM_ITERATIONS {
            let lat_lng_list = get_random_lat_lngs(num_nodes, &mut rng);

            let geocoding_start = SystemTime::now();
            let node_id_list = c_g
                .nodes_data()
                .get_node_ids_closest_to_lat_lngs(lat_lng_list);
            let end = SystemTime::now();
            let geocoding_duration = end.duration_since(geocoding_start).unwrap();
            geocoding_durations.push((geocoding_duration.as_millis() as f64) / 1000.0);

            let abstracted_graph_start = SystemTime::now();
            let abstracted_graph = form_abstracted_graph(&c_g, &node_id_list);
            let end = SystemTime::now();
            let abstracted_graph_duration = end.duration_since(abstracted_graph_start).unwrap();
            abstracted_graph_durations
                .push((abstracted_graph_duration.as_millis() as f64) / 1000.0);

            let tsp_durations_start = SystemTime::now();
            let best_path = tsp_with_repeats(&abstracted_graph, 5);
            let _p_node_ids: Vec<NodeID> = best_path
                .path
                .iter()
                .map(|node_index| {
                    *abstracted_graph
                        .nodes_data()
                        .get_node_id_by_index(node_index)
                })
                .collect();
            let end = SystemTime::now();
            let tsp_duration = end.duration_since(tsp_durations_start).unwrap();
            tsp_durations.push((tsp_duration.as_millis() as f64) / 1000.0);
        }
        println!(
            "{},{},{},{},{},{},{}",
            num_nodes,
            mean(&geocoding_durations),
            std_deviation(&geocoding_durations),
            mean(&abstracted_graph_durations),
            std_deviation(&abstracted_graph_durations),
            mean(&tsp_durations),
            std_deviation(&tsp_durations),
        );
    }
}
