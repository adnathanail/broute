#[macro_use]
extern crate rocket;

use std::ops::Deref;
use std::sync::{Arc, RwLock};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use broute::geography::datastructures::LatLng;
use broute::graphs::algorithms::{
    form_abstracted_graph, AStar, ConnectedComponents, SimulatedAnnealing,
};
use broute::graphs::datastructures::{ALDigraph, Digraph, NodeID, NodeIndex};
use broute::graphs::input::load_pbf_file;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ShortestPathResponse {
    from_point: (f64, f64),
    to_point: (f64, f64),
    path: Vec<(f64, f64)>,
    path_length: f64,
}

#[get("/shortest_path/<start_latitude>/<start_longitude>/<end_latitude>/<end_longitude>")]
fn shortest_path(
    rc: &rocket::State<RoutingCache>,
    start_latitude: f64,
    start_longitude: f64,
    end_latitude: f64,
    end_longitude: f64,
) -> Json<ShortestPathResponse> {
    let binding = rc.g.read().unwrap();
    let c_g = binding.deref();

    let start_node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(LatLng {
        latitude: start_latitude,
        longitude: start_longitude,
    });

    let end_node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(LatLng {
        latitude: end_latitude,
        longitude: end_longitude,
    });

    let mut astar = AStar::new(c_g, start_node_index, vec![end_node_index]);
    astar.run();

    println!("A* ran");

    let p = astar.get_graph_path(end_node_index).unwrap();
    // Form response
    let start_node_data = c_g.nodes_data().get_node_data_by_index(start_node_index);
    let end_node_data = c_g.nodes_data().get_node_data_by_index(end_node_index);

    let path_length = p.get_length_on_graph(c_g);

    let mut points: Vec<(f64, f64)> = vec![];
    for node_index in &p.path {
        let node_data = c_g.nodes_data().get_node_data_by_index(*node_index);
        points.push(node_data.latlng.as_lat_lng_tuple())
    }

    Json(ShortestPathResponse {
        from_point: start_node_data.latlng.as_lat_lng_tuple(),
        to_point: end_node_data.latlng.as_lat_lng_tuple(),
        path: points,
        path_length,
    })
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RouteOptimisationResponse {
    legs: Vec<Vec<(f64, f64)>>,
}

#[get("/route_optimisation/<points_str>")]
fn route_optimisation(
    rc: &rocket::State<RoutingCache>,
    points_str: &str,
) -> Json<RouteOptimisationResponse> {
    let binding = rc.g.read().unwrap();
    let c_g = binding.deref();

    println!("Graph loaded from cache");

    let lat_lng_list: Vec<LatLng> = points_str
        .split('|')
        .map(|p_str| p_str.split(',').collect())
        .map(|p: Vec<&str>| LatLng {
            latitude: p[0].parse().unwrap(),
            longitude: p[1].parse().unwrap(),
        })
        .collect();

    println!("Coords parsed");

    let mut node_id_list: Vec<NodeID> = vec![];
    for lat_lng in lat_lng_list {
        let node_index = c_g.nodes_data().get_node_index_closest_to_lat_lng(lat_lng);
        node_id_list.push(*c_g.nodes_data().get_node_id_by_index(&node_index));
    }

    let abstracted_graph = form_abstracted_graph(c_g, &node_id_list);

    println!("Abstracted graph constructed");

    let mut sa = SimulatedAnnealing::new(&abstracted_graph);
    sa.run();

    println!("TSP ran");

    let mut p_node_ids = vec![];
    for p_node_index in &sa.get_best_path().path {
        p_node_ids.push(
            abstracted_graph
                .nodes_data()
                .get_node_id_by_index(p_node_index),
        )
    }

    println!("Original graph node ID's extracted");

    let mut legs: Vec<Vec<(f64, f64)>> = vec![];
    for i in 0..(p_node_ids.len() - 1) {
        let from_node_index = c_g.nodes_data().get_node_index_by_id(p_node_ids[i]);
        let to_node_index = c_g.nodes_data().get_node_index_by_id(p_node_ids[i + 1]);
        let mut astar = AStar::new(c_g, *from_node_index, vec![*to_node_index]);
        astar.run();

        let leg_p = astar.get_graph_path(*to_node_index).unwrap();

        let mut leg: Vec<(f64, f64)> = vec![];
        for node_index in &leg_p.path {
            let node_data = c_g.nodes_data().get_node_data_by_index(*node_index);
            leg.push(node_data.latlng.as_lat_lng_tuple())
        }
        legs.push(leg)
    }
    println!("Legs reconstructed");

    Json(RouteOptimisationResponse { legs })
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
    let g = load_pbf_file("test_data/geofabrik/central-london-latest.osm.pbf").unwrap();

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
        .mount("/", routes![shortest_path, route_optimisation])
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
