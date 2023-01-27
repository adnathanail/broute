use broute::graphs::algorithms::connected_components::ConnectedComponents;
use broute::graphs::algorithms::dijkstra::dijkstra;
use broute::graphs::algorithms::travelling_salesman::{get_path_length, GraphPath};
use broute::graphs::datastructures::digraph::{Digraph, NodeIndex};
use broute::graphs::input::pbf::load_pbf_file;

fn main() {
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

    println!("Original graph {:} nodes", g.num_vertices());

    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();

    println!("Biggest connected subgraph {:} nodes", c_g.num_vertices());

    let start_node_id = c_g.nodes_data().get_node_ids()[0];
    let start_node_index = c_g.nodes_data().get_node_index_by_id(&start_node_id);
    let start_node_data = c_g.nodes_data().get_node_data_by_index(*start_node_index);

    let end_node_id = c_g.nodes_data().get_node_ids()[c_g.num_vertices() - 1];
    let end_node_index = c_g.nodes_data().get_node_index_by_id(&end_node_id);
    let end_node_data = c_g.nodes_data().get_node_data_by_index(*end_node_index);

    println!(
        "Running Dijkstra from {:},{:} to {:},{:}",
        start_node_data.longitude,
        start_node_data.latitude,
        end_node_data.longitude,
        end_node_data.latitude
    );

    let dj_out = dijkstra(&c_g, *start_node_index);

    println!("Dijkstra ran");

    let mut p = GraphPath { path: vec![] };
    let mut current_node_index = end_node_index.0;
    while current_node_index != start_node_index.0 {
        p.path.push(NodeIndex(current_node_index));
        current_node_index = dj_out.1[current_node_index].unwrap();
    }
    p.path.push(NodeIndex(current_node_index));

    println!("{:?}", p.path);

    println!("Distance {:} km", get_path_length(&c_g, &p));

    // to_svg(&g, &p, "out/paths/test3.svg");
}
