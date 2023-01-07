#[derive(Debug)]
pub struct NodeData {
    pub longitude: f64,
    pub latitude: f64,
}

pub struct DigraphAdjacency {
    pub node_index: usize,
    pub weight: f64,
}

pub trait Digraph {
    fn num_vertices(&self) -> usize;

    // Graph creation and data reading functions accept a node_id, which can be any usize value, to
    //   easily map OpenStreetMap node ID's onto my custom graph implementation
    fn add_node_data(&mut self, node_id: usize, longitude: f64, latitude: f64);

    fn add_edge(&mut self, from_id: usize, to_id: usize, weight: f64);

    fn get_node_data(&self, node_id: usize) -> &NodeData;

    // Translation between node_id and node_index is slow, so high-traffic functions (those called
    //   in the actual algorithms themselves, instead of setup/result reading) work directly with
    //   node_index
    // This should allow the shortest path/travelling salesman code to never worry about node_id's
    fn adj(&self, node_index: usize) -> Vec<DigraphAdjacency>;

    fn dist(&self, from_index: usize, to_index: usize) -> f64;
}
