use broute::geography::datastructures::LatLng;
use broute::utils::{get_random_lat_lngs, mean, std_deviation};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::time::SystemTime;

fn main() {
    let mut rng = Pcg64Mcg::from_entropy();

    println!("graph_num_nodes,duration_mean,duration_std_deviation");
    for num_nodes in [5, 10, 25, 50, 100, 250, 500, 100] {
        let mut durations: Vec<f64> = vec![];

        const NUM_ITERATIONS: i32 = 100;

        for _ in 0..NUM_ITERATIONS {
            let lat_lng_list = get_random_lat_lngs(num_nodes, &mut rng);

            let start = SystemTime::now();

            let lat_lng_string = lat_lng_list
                .iter()
                .map(|ll| format!("{},{}", ll.latitude, ll.longitude))
                .collect::<Vec<String>>()
                .join("|");

            reqwest::blocking::get(format!(
                "http://localhost:8000/route_optimisation/{lat_lng_string}/"
            ))
            .unwrap();

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
