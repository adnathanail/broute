use broute::graphs::algorithms::AStar;
use broute::graphs::datastructures::{Digraph, NodeIndex};
use broute::graphs::input::load_xgmml_file;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::time::SystemTime;

fn mean(data: &[f64]) -> f64 {
    let sum = data.iter().sum::<f64>();
    let count = data.len() as f64;

    sum / count
}

fn std_deviation(data: &[f64]) -> f64 {
    let variance = data
        .iter()
        .map(|value| {
            let diff = mean(data) - *value;

            diff * diff
        })
        .sum::<f64>()
        / data.len() as f64;

    variance.sqrt()
}

fn main() {
    let mut rng = Pcg64Mcg::from_entropy();

    println!("filename,loading_time,graph_num_nodes,path_length_mean,path_lengths_std_deviation,duration_mean,duration_std_deviation");
    for filename in [
        "USA-road-d.NY.gr",
        "USA-road-d.BAY.gr",
        "USA-road-d.COL.gr",
        "USA-road-d.FLA.gr",
        "USA-road-d.NW.gr",
        "USA-road-d.NE.gr",
        "USA-road-d.CAL.gr",
        "USA-road-d.LKS.gr",
        "USA-road-d.E.gr",
        "USA-road-d.W.gr",
        "USA-road-d.CTR.gr",
        "USA-road-d.USA.gr",
    ] {
        let loading_start = SystemTime::now();
        let g = load_xgmml_file(&format!("test_data/dimacs_shortest_path/{}", &filename)).expect(
            "Download test data from https://www.diag.uniroma1.it/~challenge9/download.shtml",
        );
        let loading_end = SystemTime::now();
        let loading_duration = loading_end.duration_since(loading_start).unwrap();

        let mut path_lengths: Vec<f64> = vec![];
        let mut durations: Vec<f64> = vec![];

        for _ in 0..100 {
            let from_node = NodeIndex(rng.gen_range(1..g.num_vertices() - 1));
            let to_node = NodeIndex(rng.gen_range(1..g.num_vertices() - 1));

            let start = SystemTime::now();

            let mut astar = AStar::new(&g, from_node, vec![to_node]);
            astar.run();

            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();

            path_lengths.push(astar.get_graph_path(to_node).unwrap().path.len() as f64);
            durations.push((duration.as_millis() as f64) / 1000.0);
        }
        println!(
            "{},{},{},{},{},{},{}",
            filename,
            (loading_duration.as_millis() as f64) / 1000.0,
            g.num_vertices(),
            mean(&path_lengths),
            std_deviation(&path_lengths),
            mean(&durations),
            std_deviation(&durations),
        );
    }
}
