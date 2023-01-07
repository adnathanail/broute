#[derive(Debug)]
pub struct NodeData {
    pub node_index: usize,
    pub longitude: f64,
    pub latitude: f64,
}

pub struct DigraphAdjacency {
    pub node_index: usize,
    pub node_longitude: f64,
    pub node_latitude: f64,
    pub weight: f64,
}

impl DigraphAdjacency {
    pub fn new(node_index: usize, node_longitude: f64, node_latitude: f64, weight: f64) -> Self {
        Self {
            node_index,
            node_longitude,
            node_latitude,
            weight,
        }
    }
}

pub trait Digraph {
    fn num_vertices(&self) -> usize;

    fn add_node_data(&mut self, node_id: usize, longitude: f64, latitude: f64);

    fn add_edge(&mut self, from_id: usize, to_id: usize, weight: f64);

    fn adj(&self, node_id: usize) -> Vec<DigraphAdjacency>;

    fn dist(&self, from_id: usize, to_id: usize) -> f64;

    fn get_node_data(&self, node_id: usize) -> NodeData;
}
