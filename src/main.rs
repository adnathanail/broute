use broute::graphs::algorithms::connected_components::connected_components;
use broute::graphs::algorithms::dijkstra::dijkstra;
use broute::graphs::algorithms::travelling_salesman::GraphPath;
use broute::graphs::datastructures::digraph::NodeIndex;
use broute::graphs::input::pbf::load_pbf_file;
use broute::graphs::output::svg::to_svg;

fn main() {
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

    connected_components(&g);
}
