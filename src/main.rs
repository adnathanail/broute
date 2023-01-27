#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::serde::json::Json;

use broute::graphs::algorithms::connected_components::ConnectedComponents;
use broute::graphs::algorithms::dijkstra::dijkstra;
use broute::graphs::algorithms::travelling_salesman::{get_path_length, GraphPath};
use broute::graphs::datastructures::digraph::{Digraph, NodeIndex};
use broute::graphs::input::pbf::load_pbf_file;

#[get("/<start_longitude>/<start_latitude>/<end_longitude>/<end_latitude>")]
fn shortest_path(start_longitude: f64,
    start_latitude: f64,
    end_longitude: f64,
    end_latitude: f64,) -> Json<Vec<(f64, f64)>> {
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

    println!("Original graph {:} nodes", g.num_vertices());

    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();

    println!("Biggest connected subgraph {:} nodes", c_g.num_vertices());

    let start_node_index = c_g
        .nodes_data()
        .get_node_index_closest_to_point(start_longitude, start_latitude);
    let start_node_data = c_g.nodes_data().get_node_data_by_index(start_node_index);

    let end_node_index = c_g
        .nodes_data()
        .get_node_index_closest_to_point(end_longitude, end_latitude);
    let end_node_data = c_g.nodes_data().get_node_data_by_index(end_node_index);

    println!(
        "Running Dijkstra from {:},{:} to {:},{:}",
        start_node_data.longitude,
        start_node_data.latitude,
        end_node_data.longitude,
        end_node_data.latitude
    );

    let dj_out = dijkstra(&c_g, start_node_index);

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

    let mut points: Vec<(f64, f64)> = vec![];
    for node_index in &p.path {
        let node_data = c_g.nodes_data().get_node_data_by_index(*node_index);
        points.push((node_data.longitude, node_data.latitude))
    }

    Json(points)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![shortest_path])
        .ignite().await?
        .launch().await?;

    Ok(())
}
