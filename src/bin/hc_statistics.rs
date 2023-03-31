use broute::graphs::algorithms::HillClimbing;
use broute::graphs::input::load_tsplib_file;
use std::time::SystemTime;

fn main() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();

    let mut tour_lengths: Vec<f64> = vec![];
    let mut durations: Vec<f64> = vec![];

    for a in 0..10 {
        println!("{a}");
        let start = SystemTime::now();
        let mut sa = HillClimbing::new(&dimacs_g);
        sa.run();

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();

        tour_lengths.push(sa.get_best_path().get_length_on_graph(&dimacs_g));
        durations.push((duration.as_millis() as f64) / 1000.0);
        println!(
            "\t{} {}s",
            tour_lengths[tour_lengths.len() - 1],
            durations[durations.len() - 1]
        );
    }
    println!(
        "Min tour length: {:?}",
        tour_lengths.iter().min_by(|a, b| a.total_cmp(b)).unwrap()
    );
    let mean_length: f64 = tour_lengths.iter().sum::<f64>() / (tour_lengths.len() as f64);
    println!("Mean tour length: {mean_length:?}");
    let mean_duration: f64 = durations.iter().sum::<f64>() / (durations.len() as f64);
    println!("Mean duration: {mean_duration:?}");
}
