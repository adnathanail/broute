use broute::graphs::algorithms::HillClimbing;
use broute::graphs::input::load_tsplib_file;
use std::fs::File;
use std::time::SystemTime;

fn main() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();

    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    let start = SystemTime::now();

    let mut sa = HillClimbing::new(&dimacs_g);
    sa.run();

    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    println!(
        "{},{}",
        sa.get_best_path().get_length_on_graph(&dimacs_g),
        (duration.as_millis() as f64) / 1000.0
    );

    if let Ok(report) = guard.report().build() {
        let file = File::create("out/flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}
