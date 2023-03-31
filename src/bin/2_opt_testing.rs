use broute::graphs::algorithms::two_opt;
use broute::graphs::datastructures::{GraphPath, NodeIndex};
use broute::graphs::input::load_tsplib_file;
use broute::graphs::output::to_svg;

fn main() {
    let dimacs_g = load_tsplib_file("test_data/dimacs_tsp/test.tsp", usize::MAX).unwrap();
    let path = GraphPath {
        path: vec![
            NodeIndex(0),
            NodeIndex(1),
            NodeIndex(2),
            NodeIndex(3),
            NodeIndex(4),
            NodeIndex(5),
            NodeIndex(6),
            NodeIndex(7),
            NodeIndex(8),
            NodeIndex(9),
        ],
    };
    to_svg(&dimacs_g, &path, "out/test.svg");
    let new_path = two_opt(&path, NodeIndex(2), NodeIndex(6));
    to_svg(&dimacs_g, &new_path, "out/test_two_opt.svg");
}
