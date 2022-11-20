// Stop the compiler warning about unused functions
#![allow(dead_code)]

use broute::graphs::{self, output::output_graph_to_file};
//use osmpbf::{Element, ElementReader};

fn main() {
    let g = graphs::random_graph::get_random_graph(5, 0.5, 4.0, 1.0);

    output_graph_to_file(g, "out/graph.png".to_string());

//    println!("{}", g.get_graphviz_string());
//    println!("{}", g);

//    println!("{:?}", graphs::dijkstra::dijkstra(&g));

//    let reader = ElementReader::from_path("theoffice-latest.osm.pbf").unwrap();
//    let mut nodes = 0_u64;
//    let mut dense_nodes = 0_u64;
//    let mut ways = 0_u64;
//    let mut relations = 0_u64;
//    let _ = reader.for_each(|element| {
//        match element {
//            Element::Node(node) => {
//                if nodes == 0 {
//                    println!("Node");
//                    println!("{:?}", node);
//                }
//                nodes += 1;
//            }
//            Element::DenseNode(dense_node) => {
//                if dense_nodes == 0 {
//                    println!("DenseNode");
//                    println!("{:?}", dense_node);
//                }
//                dense_nodes += 1;
//            }
//            Element::Way(way) => {
//                if ways == 0 {
//                    println!("Way");
//                    println!("{:?}", way);
//                }
//                ways += 1;
//            }
//            Element::Relation(relation) => {
//                if relations == 0 {
//                    println!("Relation");
//                    println!("{:?}", relation);
//                }
//                relations += 1;
//            }
//        }
//    });
//
//    println!("Number of ways: {ways}");
}
