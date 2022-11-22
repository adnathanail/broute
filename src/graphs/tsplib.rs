use std::{cmp, io::Cursor};
use tsplib::NodeCoord;

use super::digraph::Digraph;

pub fn load_tsplib_file(input_data: String, num_nodes: usize) -> Digraph {
    let instance = tsplib::parse(Cursor::new(&input_data[..])).unwrap();

    let actual_num_nodes = cmp::min(num_nodes, instance.dimension);

    let coords = match instance.node_coord.unwrap() {
        NodeCoord::Two(x) => x,
        _ => panic!("Wrong format"),
    };

    let mut g = Digraph::new(actual_num_nodes);

    for i in 0..actual_num_nodes {
        for j in 0..actual_num_nodes {
            if coords[i].0 != coords[j].0 {
                let dx = coords[j].1 - coords[i].1;
                let dy = coords[j].2 - coords[i].2;
                let weight = (f32::powf(dx, 2.0) + f32::powf(dy, 2.0)).sqrt();
                g.add_edge(coords[i].0 - 1, coords[j].0 - 1, weight) // -1 because TSPLIB is 1 indexed
            }
        }
    }

    g
}
