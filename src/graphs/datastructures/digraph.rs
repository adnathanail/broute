#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeIndex(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeID(pub usize);

#[derive(Debug)]
pub struct NodeData {
    pub longitude: f64,
    pub latitude: f64,
}

pub struct DigraphAdjacency {
    pub node_index: NodeIndex,
    pub weight: f64,
}

pub trait Digraph {
    fn num_vertices(&self) -> usize;

    // Graph creation and data reading functions accept a node_id, which can be any usize value, to
    //   easily map OpenStreetMap node ID's onto my custom graph implementation
    fn add_node_data(&mut self, node_id: NodeID, longitude: f64, latitude: f64);

    fn add_edge(&mut self, from_id: NodeID, to_id: NodeID, weight: f64);

    fn get_node_data(&self, node_id: NodeID) -> &NodeData;

    // Translation between node_id and node_index is slow, so high-traffic functions (those called
    //   in the actual algorithms themselves, instead of setup/result reading) work directly with
    //   node_index
    // This should allow the shortest path/travelling salesman code to never worry about node_id's
    fn adj(&self, node_index: NodeIndex) -> Vec<DigraphAdjacency>;

    fn dist(&self, from_index: NodeIndex, to_index: NodeIndex) -> f64;
}
