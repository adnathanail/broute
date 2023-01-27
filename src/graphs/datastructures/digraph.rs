use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeIndex(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeID(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct NodeData {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug)]
pub struct NodesData {
    node_data: HashMap<NodeIndex, NodeData>,
    node_id_index_lookup: HashMap<NodeID, NodeIndex>,
    node_index_id_lookup: HashMap<NodeIndex, NodeID>,
    current_node_index: NodeIndex,
}

impl NodesData {
    pub fn new() -> Self {
        Self {
            node_data: HashMap::new(),
            node_id_index_lookup: HashMap::new(),
            node_index_id_lookup: HashMap::new(),
            current_node_index: NodeIndex(0),
        }
    }

    pub fn add_node_data_by_parts(&mut self, node_id: NodeID, longitude: f64, latitude: f64) {
        self.add_node_data(
            node_id,
            NodeData {
                longitude,
                latitude,
            },
        );
    }

    pub fn add_node_data(&mut self, node_id: NodeID, node_data: NodeData) {
        self.node_id_index_lookup
            .insert(node_id, self.current_node_index);
        self.node_index_id_lookup
            .insert(self.current_node_index, node_id);
        self.current_node_index.0 += 1;
        self.node_data
            .insert(self.node_id_index_lookup[&node_id], node_data);
    }

    pub fn get_node_ids(&self) -> Vec<NodeID> {
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

    pub fn get_node_id_by_index(&self, node_index: &NodeIndex) -> &NodeID {
        self.node_index_id_lookup.get(node_index).unwrap()
    }
}

impl Default for NodesData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct DigraphAdjacency {
    pub node_index: NodeIndex,
    pub weight: f64,
}

pub trait Digraph {
    fn num_vertices(&self) -> usize;

    fn add_edge_by_id(&mut self, from_id: NodeID, to_id: NodeID, weight: f64) {
        self.add_edge_by_index(
            *self.nodes_data().get_node_index_by_id(&from_id),
            *self.nodes_data().get_node_index_by_id(&to_id),
            weight,
        )
    }

    fn add_edge_by_index(&mut self, from_index: NodeIndex, to_index: NodeIndex, weight: f64);

    fn adj(&self, node_index: NodeIndex) -> Vec<DigraphAdjacency>;

    fn dist(&self, from_index: NodeIndex, to_index: NodeIndex) -> f64;

    fn nodes_data(&self) -> &NodesData;

    fn mut_nodes_data(&mut self) -> &mut NodesData;
}
