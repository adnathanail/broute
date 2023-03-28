use broute::graphs::algorithms::SimulatedAnnealing;
use broute::graphs::input::load_tsplib_file;
use std::time::SystemTime;

fn main() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();

    println!("T0,N,alpha,Tmin,length,duration");
    for a in [0.95, 0.99, 0.995] {
        for i in [1, 5, 10] {
            for t in [1.0, 10.0, 100.0, 1000.0, 10000.0] {
                for tmin in [1e-1_f64, 1e-3_f64, 1e-5_f64, 1e-7_f64, 1e-9_f64] {
                    let start = SystemTime::now();
                    let mut sa = SimulatedAnnealing::new_with_custom_parameters(&dimacs_g, t, i, a, tmin);
                    sa.run();
                    let end = SystemTime::now();
                    let duration = end.duration_since(start).unwrap();
                    println!("{},{},{},{},{},{}", t, i, a, tmin, sa.get_best_path().get_length_on_graph(&dimacs_g), (duration.as_millis() as f64) / 1000.0);
                }
            }
        }
    }
}
