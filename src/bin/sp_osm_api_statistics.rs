use broute::utils::{get_random_lat_lngs, mean, std_deviation};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use std::time::SystemTime;

fn main() {
    let mut rng = Pcg64Mcg::from_entropy();

    println!(
        "original_graph_num_nodes,connected_graph_num_nodes,duration_mean,duration_std_deviation"
    );
    let mut durations: Vec<f64> = vec![];

    const NUM_ITERATIONS: i32 = 100;

    for _ in 0..NUM_ITERATIONS {
        let lat_lng_list = get_random_lat_lngs(2, &mut rng);

        let start = SystemTime::now();

        reqwest::blocking::get(format!(
            "http://localhost:8000/shortest_path/{}/{}/{}/{}/",
            lat_lng_list[0].latitude,
            lat_lng_list[0].longitude,
            lat_lng_list[1].latitude,
            lat_lng_list[1].longitude
        ))
        .unwrap();

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();

        durations.push((duration.as_millis() as f64) / 1000.0);
    }
    println!(",,{},{}", mean(&durations), std_deviation(&durations),);
}
