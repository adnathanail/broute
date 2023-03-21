use broute::graphs::algorithms::SimulatedAnnealing;
use broute::graphs::input::load_tsplib_file;
use std::time::SystemTime;

fn main() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();

    for a in [0.9995] {
        for i in [50, 100, 500, 1000] {
            println!("100 {a} {i}");
            let start = SystemTime::now();
            let mut sa = SimulatedAnnealing::new(&dimacs_g);
            sa.run(100.0, a, i);
            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();
            println!("\t{} seconds", duration.as_secs());
        }
    }
}
