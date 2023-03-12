use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

use crate::graphs::datastructures::{ALDigraph, Digraph, LatLng, NodeID, NodeIndex};

#[derive(Debug)]
pub enum XGMMLImportError {
    IOError(std::io::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    OtherError(String),
}

impl From<std::io::Error> for XGMMLImportError {
    fn from(err: std::io::Error) -> XGMMLImportError {
        XGMMLImportError::IOError(err)
    }
}

impl From<std::num::ParseIntError> for XGMMLImportError {
    fn from(err: std::num::ParseIntError) -> XGMMLImportError {
        XGMMLImportError::ParseInt(err)
    }
}

impl From<std::num::ParseFloatError> for XGMMLImportError {
    fn from(err: std::num::ParseFloatError) -> XGMMLImportError {
        XGMMLImportError::ParseFloat(err)
    }
}

type Result<T> = std::result::Result<T, XGMMLImportError>;

pub fn load_xgmml_file(file_path: &str) -> Result<ALDigraph> {
    let mut opt_num_nodes: Option<usize> = None;
    let file = File::open(file_path)?;
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let ip = line?;
        let parts = ip.split(" ").collect::<Vec<&str>>();
        if parts[0] == "p" {
            opt_num_nodes = parts[2].parse::<usize>().ok();
            break;
        }
    }
    let Some(num_nodes) = opt_num_nodes else {
        return Err(XGMMLImportError::OtherError("Missing p line defining number of nodes".to_string()));
    };

    let mut g = ALDigraph::new(num_nodes);
    for i in 1..(num_nodes + 1) {
        g.mut_nodes_data().add_node_data_by_parts(
            NodeID(i),
            LatLng { latitude: 0.0, longitude: 0.0 },  // Lat lng aren't given in this dataset
        );
    }

    let file = File::open(file_path)?;
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let ip = line?;
        let parts = ip.split(" ").collect::<Vec<&str>>();
        if parts[0] == "a" {
            g.add_edge_by_index(NodeIndex(parts[1].parse::<usize>()? - 1), NodeIndex(parts[2].parse::<usize>()? - 1), parts[3].parse::<f64>()?);
        }
    }

    Ok(g)
}
