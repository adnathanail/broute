use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeIndex(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeID(pub usize);

#[derive(Debug)]
pub struct NodeData {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug)]
pub struct NodesData {
    node_data: HashMap<NodeIndex, NodeData>,
    node_id_index_lookup: HashMap<NodeID, NodeIndex>,
    current_node_index: NodeIndex,
}

impl NodesData {
    pub fn new() -> Self {
        Self {
            node_data: HashMap::new(),
            node_id_index_lookup: HashMap::new(),
            current_node_index: NodeIndex(0),
        }
    }

    pub fn add_node_data(&mut self, node_id: NodeID, longitude: f64, latitude: f64) {
        self.node_id_index_lookup
            .insert(node_id, self.current_node_index);
        self.current_node_index.0 += 1;
        self.node_data.insert(
            self.node_id_index_lookup[&node_id],
            NodeData {
                longitude,
                latitude,
            },
        );
    }

    pub fn get_node_ids(&self) -> Vec<NodeID> {
        // Return the list of node ids
        self.node_id_index_lookup.keys().cloned().collect()
    }

    pub fn get_node_data_by_id(&self, node_id: NodeID) -> &NodeData {
        self.node_data
            .get(self.node_id_index_lookup.get(&node_id).unwrap())
            .unwrap()
    }

    pub fn get_node_data_by_index(&self, node_index: NodeIndex) -> &NodeData {
        self.node_data.get(&node_index).unwrap()
    }

    pub fn get_node_index_by_id(&self, node_id: &NodeID) -> &NodeIndex {
        self.node_id_index_lookup.get(node_id).unwrap()
    }
}

impl Default for NodesData {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DigraphAdjacency {
    pub node_index: NodeIndex,
    pub weight: f64,
}

pub trait Digraph {
    fn num_vertices(&self) -> usize;

    // Graph creation and data reading functions accept a node_id, which can be any usize value, to
    //   easily map OpenStreetMap node ID's onto my custom graph implementation
    fn add_edge(&mut self, from_id: NodeID, to_id: NodeID, weight: f64);

    // Translation between node_id and node_index is slow, so high-traffic functions (those called
    //   in the actual algorithms themselves, instead of setup/result reading) work directly with
    //   node_index
    // This should allow the shortest path/travelling salesman code to never worry about node_id's
    fn adj(&self, node_index: NodeIndex) -> Vec<DigraphAdjacency>;

    fn dist(&self, from_index: NodeIndex, to_index: NodeIndex) -> f64;

    fn nodes_data(&self) -> &NodesData;

    fn mut_nodes_data(&mut self) -> &mut NodesData;
}
