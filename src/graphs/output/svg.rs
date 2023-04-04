use crate::graphs::datastructures::{Digraph, GraphPath, NodeIndex};
use svg::node::element::path::Data;
use svg::node::element::{Circle, Path};
use svg::{Document, Node};

fn get_points_bounds(g: &impl Digraph) -> (f64, f64, f64, f64) {
    // Draw the locations of the node on the canvas, with longitudes and latitudes normalised
    // to the range 0-100
    let mut min_lon: f64 = f64::MAX;
    let mut max_lon: f64 = f64::MIN;
    let mut min_lat: f64 = f64::MAX;
    let mut max_lat: f64 = f64::MIN;
    for node_id in g.nodes_data().get_node_ids() {
        let nd = g.nodes_data().get_node_data_by_id(node_id);

        if nd.latlng.longitude < min_lon {
            min_lon = nd.latlng.longitude;
        }
        if nd.latlng.longitude > max_lon {
            max_lon = nd.latlng.longitude;
        }
        if nd.latlng.latitude < min_lat {
            min_lat = nd.latlng.latitude;
        }
        if nd.latlng.latitude > max_lat {
            max_lat = nd.latlng.latitude;
        }
    }
    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    (min_lon, min_lat, lon_range, lat_range)
}

fn normalise_point(
    min_lon: f64,
    min_lat: f64,
    lon_range: f64,
    lat_range: f64,
    longitude: f64,
    latitude: f64,
) -> (f64, f64) {
    let x = ((longitude - min_lon) / lon_range) * OUTPUT_WIDTH;
    let y = ((latitude - min_lat) / lat_range) * OUTPUT_HEIGHT;
    (x, y)
}

fn get_coords_from_node_index(
    g: &impl Digraph,
    node_index: NodeIndex,
    min_lon: f64,
    min_lat: f64,
    lon_range: f64,
    lat_range: f64,
) -> (f64, f64) {
    let nd = g.nodes_data().get_node_data_by_index(node_index);
    normalise_point(
        min_lon,
        min_lat,
        lon_range,
        lat_range,
        nd.latlng.longitude,
        nd.latlng.latitude,
    )
}

const OUTPUT_WIDTH: f64 = 100.0;
const OUTPUT_HEIGHT: f64 = 100.0;

/// Export a Digraph and GraphPath to an SVG file at the specified output_path
pub fn to_svg(g: &impl Digraph, path: &GraphPath, output_path: &str) {
    let mut document = Document::new().set(
        "viewBox",
        (-50, -50, OUTPUT_WIDTH + 50.0, OUTPUT_HEIGHT + 50.0),
    );

    let mut data = Data::new();

    let (min_lon, min_lat, lon_range, lat_range) = get_points_bounds(g);

    for index in 0..g.num_vertices() {
        let (x, y) =
            get_coords_from_node_index(g, NodeIndex(index), min_lon, min_lat, lon_range, lat_range);

        let sp = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 5)
            .set("fill", "red");

        let mut text = svg::node::element::Text::new()
            .set("x", x)
            .set("y", y)
            .set("fill", "white");
        text.append(svg::node::Text::new(format!("{index}")));

        document = document.add(sp);
        document = document.add(text);
    }

    for point in &path.path {
        let (x, y) = get_coords_from_node_index(g, *point, min_lon, min_lat, lon_range, lat_range);

        if data.len() == 0 {
            data = data.move_to((x, y));
        } else {
            data = data.line_to((x, y));
        }
    }
    let (x, y) =
        get_coords_from_node_index(g, path.path[0], min_lon, min_lat, lon_range, lat_range);
    data = data.line_to((x, y));

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    document = document.add(path);

    svg::save(output_path, &document).expect("Failed to save");
}

#[cfg(test)]
mod tests {
    use crate::geography::datastructures::LatLng;
    use crate::graphs::datastructures::{ALDigraph, Digraph, GraphPath, NodeID, NodeIndex};
    use crate::graphs::output::to_svg;
    use std::fs;

    fn get_test_graph() -> ALDigraph {
        let mut g = ALDigraph::new(5);

        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(0),
            LatLng {
                latitude: 1.0,
                longitude: 0.0,
            },
        );

        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(1),
            LatLng {
                latitude: 0.0,
                longitude: 1.0,
            },
        );

        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(2),
            LatLng {
                latitude: 2.0,
                longitude: 1.0,
            },
        );

        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(3),
            LatLng {
                latitude: 0.0,
                longitude: 3.0,
            },
        );

        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(4),
            LatLng {
                latitude: 3.0,
                longitude: 2.0,
            },
        );

        g.add_edge_by_id(NodeID(1), NodeID(0), 1.0);
        g.add_edge_by_id(NodeID(0), NodeID(2), 1.0);
        g.add_edge_by_id(NodeID(2), NodeID(1), 1.0);
        g.add_edge_by_id(NodeID(0), NodeID(3), 1.0);
        g.add_edge_by_id(NodeID(3), NodeID(4), 1.0);

        g
    }

    fn get_test_graph_path() -> GraphPath {
        GraphPath {
            path: vec![
                NodeIndex(1),
                NodeIndex(0),
                NodeIndex(2),
                NodeIndex(1),
                NodeIndex(0),
                NodeIndex(3),
                NodeIndex(4),
            ],
        }
    }

    #[test]
    fn to_svg_test() {
        to_svg(
            &get_test_graph(),
            &get_test_graph_path(),
            "out/to_svg_test.svg",
        );
        assert!(fs::read_to_string("out/to_svg_test.svg")
        .unwrap()
        .contains("M33.333332,0 L0,33.333332 L33.333332,66.666664 L33.333332,0 L0,33.333332 L100,0 L66.666664,100 L33.333332,0"));
    }
}
