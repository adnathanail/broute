use crate::geography::algorithms::haversine;
use crate::geography::datastructures::LatLng;
use std::collections::HashMap;

/// Struct used to store the index of a node in a graph,
/// this will be different in different graphs
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeIndex(pub usize);

/// Struct used to store the ID of a node in a graph,
/// this should be consistent everywhere that the "same" node is added to a graph
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeID(pub usize);

/// Struct used to hold data about a single node in a graph
#[derive(Debug, Copy, Clone)]
pub struct NodeData {
    /// Latitude and longitude of the node
    pub latlng: LatLng,
}

/// Struct used to hold data about nodes in a graph, including `NodeIndex` <-> `NodeID` lookups, and `NodeData`s
#[derive(Debug)]
pub struct NodesData {
    node_data: HashMap<NodeIndex, NodeData>,
    node_id_index_lookup: HashMap<NodeID, NodeIndex>,
    node_index_id_lookup: HashMap<NodeIndex, NodeID>,
    current_node_index: NodeIndex,
}

impl NodesData {
    /// Instantiate a new `NodesData` struct
    pub fn new() -> Self {
        Self {
            node_data: HashMap::new(),
            node_id_index_lookup: HashMap::new(),
            node_index_id_lookup: HashMap::new(),
            current_node_index: NodeIndex(0),
        }
    }

    /// Add a node to the graph (required before adding edges)
    pub fn add_node_data_by_parts(&mut self, node_id: NodeID, latlng: LatLng) {
        self.add_node_data(node_id, NodeData { latlng });
    }

    /// Add a node to the graph passing a `NodeData` struct
    pub fn add_node_data(&mut self, node_id: NodeID, node_data: NodeData) {
        self.node_id_index_lookup
            .insert(node_id, self.current_node_index);
        self.node_index_id_lookup
            .insert(self.current_node_index, node_id);
        self.current_node_index.0 += 1;
        self.node_data
            .insert(self.node_id_index_lookup[&node_id], node_data);
    }

    /// Get a vector of all `NodeIndex`s in the graph
    pub fn get_node_indexes(&self) -> Vec<NodeIndex> {
        (0..self.current_node_index.0).map(NodeIndex).collect()
    }

    /// Get a vector of all `NodeID`s in the graph
    pub fn get_node_ids(&self) -> Vec<NodeID> {
        self.node_id_index_lookup.keys().cloned().collect()
    }

    /// Get the `NodeData` of a node given its `NodeID`
    pub fn get_node_data_by_id(&self, node_id: NodeID) -> &NodeData {
        self.node_data
            .get(self.node_id_index_lookup.get(&node_id).unwrap())
            .unwrap()
    }

    /// Get the `NodeData` of a node given its `NodeIndex`
    pub fn get_node_data_by_index(&self, node_index: NodeIndex) -> &NodeData {
        self.node_data.get(&node_index).unwrap()
    }

    /// Get the `NodeIndex` of a node given its `NodeID`
    pub fn get_node_index_by_id(&self, node_id: &NodeID) -> &NodeIndex {
        self.node_id_index_lookup.get(node_id).unwrap()
    }

    /// Get the `NodeID` of a node given its `NodeIndex`
    pub fn get_node_id_by_index(&self, node_index: &NodeIndex) -> &NodeID {
        self.node_index_id_lookup.get(node_index).unwrap()
    }

    /// Get the `NodeIndex` of the node closest to a given `LatLng`
    pub fn get_node_index_closest_to_lat_lng(&self, lat_lng: LatLng) -> NodeIndex {
        let mut closest_node_index = NodeIndex(0);
        let mut closest_node_distance = f64::MAX;
        for (node_index, node_data) in self.node_data.iter() {
            let distance = haversine(lat_lng, node_data.latlng);
            if distance < closest_node_distance {
                closest_node_distance = distance;
                closest_node_index = *node_index;
            }
        }
        closest_node_index
    }

    /// Get a vector of `NodeID`s of the nodes closest to the given list of `LatLng`s
    pub fn get_node_ids_closest_to_lat_lngs(&self, lat_lngs: Vec<LatLng>) -> Vec<NodeID> {
        lat_lngs
            .iter()
            .map(|lat_lng| {
                *self.get_node_id_by_index(&self.get_node_index_closest_to_lat_lng(*lat_lng))
            })
            .collect()
    }
}

impl Default for NodesData {
    fn default() -> Self {
        Self::new()
    }
}

/// Struct used to represent an edge on a `Digraph`
#[derive(Debug)]
pub struct DigraphAdjacency {
    /// Index of the adjacent node
    pub node_index: NodeIndex,
    /// Weight of the edge from the node to the adjacent node (typically distance)
    pub weight: f64,
}

/// Trait representing a directed graph, independent of the underlying data structure
pub trait Digraph {
    /// Get the number of nodes in a `Digraph`
    fn num_vertices(&self) -> usize;

    /// Add an edge to a `Digraph` by `NodeID`s
    fn add_edge_by_id(&mut self, from_id: NodeID, to_id: NodeID, weight: f64) {
        self.add_edge_by_index(
            *self.nodes_data().get_node_index_by_id(&from_id),
            *self.nodes_data().get_node_index_by_id(&to_id),
            weight,
        )
    }

    /// Add an edge to a `Digraph` by `NodeIndex`s
    fn add_edge_by_index(&mut self, from_index: NodeIndex, to_index: NodeIndex, weight: f64);

    /// Get a vector of the nodes adjacent to a given `NodeIndex` on a `Digraph`
    fn adj(&self, node_index: NodeIndex) -> Vec<DigraphAdjacency>;

    /// Get the distance between two `NodeIndex`s on a `Digraph`
    fn dist(&self, from_index: NodeIndex, to_index: NodeIndex) -> f64;

    /// Access an immutable copy of a `Digraph`s `NodesData` object
    fn nodes_data(&self) -> &NodesData;

    /// Access a mutable copy of a `Digraph`s `NodesData` object
    fn mut_nodes_data(&mut self) -> &mut NodesData;
}
