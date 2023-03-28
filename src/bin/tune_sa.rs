use broute::graphs::algorithms::HillClimbing;
use broute::graphs::input::load_tsplib_file;
use std::time::SystemTime;

fn main() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();

    println!("alpha,N,num_iterations,length,duration");
    for a in [0.99, 0.995, 0.999, 0.9995] {
        for i in [5, 10, 50, 100] {
            for t0 in [10.0] {
                for tmin in [1e-5] {
                    let num_iterations =
                        i * ((((tmin / t0) as f64).log10() / (a as f64).log10()).ceil() as usize);
                    let start = SystemTime::now();
                    let mut sa =
                        HillClimbing::new_with_custom_parameters(&dimacs_g, num_iterations);
                    sa.run();
                    let end = SystemTime::now();
                    let duration = end.duration_since(start).unwrap();
                    println!(
                        "{},{},{},{},{}",
                        a,
                        i,
                        num_iterations,
                        sa.get_best_path().get_length_on_graph(&dimacs_g),
                        (duration.as_millis() as f64) / 1000.0
                    );
                }
            }
        }
    }
}
