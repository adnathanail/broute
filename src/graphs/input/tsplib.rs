use crate::geography::datastructures::LatLng;
use crate::graphs::datastructures::{AMDigraph, Digraph, NodeID};
use rand::{thread_rng, Rng};
use std::{cmp, fs, io::Cursor};
use tsplib::NodeCoord;

#[derive(Debug)]
pub enum TSPLIBImportError {
    IOError(std::io::Error),
    OtherError(String),
}

impl From<std::io::Error> for TSPLIBImportError {
    fn from(err: std::io::Error) -> TSPLIBImportError {
        TSPLIBImportError::IOError(err)
    }
}

type Result<T> = std::result::Result<T, TSPLIBImportError>;

pub fn load_tsplib_file(file_path: &str, num_nodes: usize) -> Result<AMDigraph> {
    let tsp_string = fs::read_to_string(file_path)?;
    let instance = tsplib::parse(Cursor::new(&tsp_string[..]))?;

    let actual_num_nodes = cmp::min(num_nodes, instance.dimension);

    let coords = match instance.node_coord.ok_or(TSPLIBImportError::OtherError(
        "No node coords found".to_string(),
    ))? {
        NodeCoord::Two(x) => Ok(x),
        _ => Err(TSPLIBImportError::OtherError(
            "No node coords found".to_string(),
        )),
    }?;

    let mut g = AMDigraph::new(actual_num_nodes);

    for (i, coord) in coords.iter().enumerate().take(actual_num_nodes) {
        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(i),
            LatLng {
                latitude: coord.0 as f64,
                longitude: coord.1 as f64,
            },
        );
    }

    for i in 0..actual_num_nodes {
        for j in 0..actual_num_nodes {
            if coords[i].0 != coords[j].0 {
                let dx = coords[j].1 - coords[i].1;
                let dy = coords[j].2 - coords[i].2;
                let weight = (f64::powf(dx as f64, 2.0) + f64::powf(dy as f64, 2.0)).sqrt();
                g.add_edge_by_id(NodeID(coords[i].0 - 1), NodeID(coords[j].0 - 1), weight)
                // -1 because TSPLIB is 1 indexed
            }
        }
    }

    Ok(g)
}

pub fn generate_random_tsplib_file(num_nodes: usize) -> String {
    let mut rng = thread_rng();

    let mut lines: Vec<String> = Vec::with_capacity(num_nodes);

    lines.push("NAME : example".to_string());
    lines.push("TYPE : TSP".to_string());
    lines.push(format!("DIMENSION : {}", num_nodes + 1));
    lines.push("EDGE_WEIGHT_TYPE: EUC_2D".to_string());
    lines.push("NODE_COORD_SECTION".to_string());

    let grid_size = num_nodes * 10;
    for i in 0..num_nodes {
        lines.push(format!(
            "{} {} {}",
            i,
            rng.gen::<f32>() * (grid_size as f32),
            rng.gen::<f32>() * (grid_size as f32)
        ))
    }

    lines.push("EOF".to_string());

    lines.join("\n")
}
