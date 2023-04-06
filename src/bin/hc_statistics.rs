use broute::graphs::algorithms::HillClimbing;
use broute::graphs::datastructures::Digraph;
use broute::graphs::input::load_tsplib_file;
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
    // let paths = fs::read_dir("test_data/dimacs_tsp/big").unwrap();
    //
    // for path in paths {
    //     println!(
    //         "{:?},",
    //         path.unwrap()
    //             .path()
    //             .display()
    //             .to_string()
    //             .split("/")
    //             .last()
    //             .unwrap()
    //     );
    // }

    println!("filename,loading_time,graph_num_nodes,num_iterations,path_length_mean,path_lengths_std_deviation,duration_mean,duration_std_deviation");
    for filename in [
        "dsj1000.tsp",
        "pr1002.tsp",
        "u1060.tsp",
        "vm1084.tsp",
        "pcb1173.tsp",
        "d1291.tsp",
        "rl1304.tsp",
        "rl1323.tsp",
        "nrw1379.tsp",
        "fl1400.tsp",
        "u1432.tsp",
        "fl1577.tsp",
        "d1655.tsp",
        "vm1748.tsp",
        "u1817.tsp",
        "rl1889.tsp",
        "d2103.tsp",
        "u2152.tsp",
        "u2319.tsp",
        "pr2392.tsp",
        "pcb3038.tsp",
        "fl3795.tsp",
        "fnl4461.tsp",
        "rl5915.tsp",
        "rl5934.tsp",
        "pla7397.tsp",
        "rl11849.tsp",
        "usa13509.tsp",
        "brd14051.tsp",
        "d15112.tsp",
        "d18512.tsp",
        "pla33810.tsp",
    ] {
        let loading_start = SystemTime::now();
        let g = load_tsplib_file(&format!("test_data/dimacs_tsp/big/{}", &filename), usize::MAX).expect(
            "Download test data from http://dimacs.rutgers.edu/archive/Challenges/TSP/download.html",
        );
        let loading_end = SystemTime::now();
        let loading_duration = loading_end.duration_since(loading_start).unwrap();

        let mut path_lengths: Vec<f64> = vec![];
        let mut durations: Vec<f64> = vec![];

        const NUM_ITERATIONS: i32 = 100;

        for _ in 0..NUM_ITERATIONS {
            let start = SystemTime::now();

            let mut hc = HillClimbing::new(&g);
            hc.run();

            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();

            path_lengths.push(hc.get_best_path().get_length_on_graph(&g));
            durations.push((duration.as_millis() as f64) / 1000.0);
        }
        println!(
            "{},{},{},{},{},{},{},{}",
            filename,
            (loading_duration.as_millis() as f64) / 1000.0,
            g.num_vertices(),
            NUM_ITERATIONS,
            mean(&path_lengths),
            std_deviation(&path_lengths),
            mean(&durations),
            std_deviation(&durations),
        );
    }
}
