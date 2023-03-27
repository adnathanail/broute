use broute::graphs::algorithms::SimulatedAnnealing;
use broute::graphs::input::load_tsplib_file;
use std::time::SystemTime;

fn main() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();

    for a in 0..10 {
        println!("{a}");
        let start = SystemTime::now();
        let mut sa = SimulatedAnnealing::new(&dimacs_g);
        sa.run();

        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();

        println!("\t{}", sa.get_best_path().get_length_on_graph(&dimacs_g));
        println!("\t{} seconds", duration.as_secs());
    }
}
