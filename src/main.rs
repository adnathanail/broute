#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use broute::graphs::algorithms::connected_components::ConnectedComponents;
use broute::graphs::algorithms::dijkstra::Dijkstra;
use broute::graphs::datastructures::al_digraph::ALDigraph;
use broute::graphs::datastructures::digraph::Digraph;
use broute::graphs::input::pbf::load_pbf_file;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ShortestPathResponse {
    from_point: (f64, f64),
    to_point: (f64, f64),
    path: Vec<(f64, f64)>,
    path_length: f64,
}

#[get("/<start_latitude>/<start_longitude>/<end_latitude>/<end_longitude>")]
fn shortest_path(
    rc: &rocket::State<RoutingCache>,
    start_latitude: f64,
    start_longitude: f64,
    end_latitude: f64,
    end_longitude: f64,
) -> Json<ShortestPathResponse> {
    let binding = rc.g.read().unwrap();
    let c_g = binding.deref();

    let start_node_index = c_g
        .nodes_data()
        .get_node_index_closest_to_point(start_latitude, start_longitude);

    let end_node_index = c_g
        .nodes_data()
        .get_node_index_closest_to_point(end_latitude, end_longitude);

    let mut dj = Dijkstra::new(c_g, start_node_index);
    dj.run();

    println!("Dijkstra ran");

    let p = dj.get_graph_path(end_node_index);
    // Form response
    let start_node_data = c_g.nodes_data().get_node_data_by_index(start_node_index);
    let end_node_data = c_g.nodes_data().get_node_data_by_index(end_node_index);

    let path_length = p.get_length_on_graph(c_g);

    let mut points: Vec<(f64, f64)> = vec![];
    for node_index in &p.path {
        let node_data = c_g.nodes_data().get_node_data_by_index(*node_index);
        points.push((node_data.latitude, node_data.longitude))
    }

    Json(ShortestPathResponse {
        from_point: (start_node_data.latitude, start_node_data.longitude),
        to_point: (end_node_data.latitude, end_node_data.longitude),
        path: points,
        path_length,
    })
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

async fn get_graph() -> ALDigraph {
    let g = load_pbf_file("test_data/geofabrik/monaco-latest.osm.pbf");

    println!("Original graph {:} nodes", g.num_vertices());

    let mut cc = ConnectedComponents::new(&g);
    cc.run();
    let c_g = cc.get_largest_connected_subgraphs();

    println!("Biggest connected subgraph {:} nodes", c_g.num_vertices());

    c_g
}

struct RoutingCache {
    // https://stackoverflow.com/questions/68908091/how-do-i-send-read-only-data-to-other-threads-without-copying
    g: Arc<RwLock<ALDigraph>>,
}

async fn rocket() -> Result<rocket::Rocket<rocket::Ignite>, rocket::Error> {
    let c_g = get_graph().await;

    rocket::build()
        .mount("/", routes![shortest_path])
        .attach(CORS)
        .manage(RoutingCache {
            g: Arc::new(RwLock::new(c_g)),
        })
        .ignite()
        .await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket().await?.launch().await?;

    Ok(())
}
