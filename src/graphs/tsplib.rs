use tsplib::{EdgeWeightType, NodeCoord, Type};
use std::io::Cursor;

use super::digraph::Digraph;

pub fn load_tsplib_file(input_data: String) -> Digraph {
    let instance = tsplib::parse(Cursor::new(&input_data[..])).unwrap();

    let coords = match instance.node_coord.unwrap() {
        NodeCoord::Two(x) => x,
        _ => panic!("Wrong format")
    };

    let mut g = Digraph::new(instance.dimension);

    for coord1 in coords.iter() {
        for coord2 in coords.iter() {
            if coord1.0 != coord2.0 {
                let dx = coord2.1 - coord1.1;
                let dy = coord2.2 - coord1.2;
                let weight = (f32::powf(dx, 2.0) + f32::powf(dy, 2.0)).sqrt();
                g.add_edge(coord1.0 - 1, coord2.0 - 1, weight)  // -1 because TSPLIB is 1 indexed
            }
        }
    }

    g
}