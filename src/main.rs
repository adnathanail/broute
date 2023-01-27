use broute::graphs::algorithms::dijkstra::dijkstra;
use broute::graphs::algorithms::travelling_salesman::GraphPath;
use broute::graphs::datastructures::digraph::NodeIndex;
use broute::graphs::input::pbf::load_pbf_file;
use broute::graphs::output::svg::to_svg;

fn main() {
    let g = load_pbf_file("test_data/geofabrik/house.osm.pbf");

    // println!("{:?}", g.nodes_data().get_node_ids());
    // let start_node_id = NodeID(18446744073709413154);
    // let start_node_index = g.nodes_data().get_node_index_by_id(&start_node_id);
    // let end_node_id = NodeID(18446744073709413143);
    // let end_node_index = g.nodes_data().get_node_index_by_id(&end_node_id);

    let start_node_index = &NodeIndex(6);
    let end_node_index = &NodeIndex(19);

    let dj_out = dijkstra(&g, *start_node_index);

    let mut p = GraphPath { path: vec![] };
    let mut current_node_index = end_node_index.0;
    while current_node_index != start_node_index.0 {
        p.path.push(NodeIndex(current_node_index));
        current_node_index = dj_out.1[current_node_index].unwrap();
    }
    p.path.push(NodeIndex(current_node_index));

    to_svg(&g, &p, "out/paths/test3.svg");
}
