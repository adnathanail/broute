#[derive(Debug)]
pub struct Digraph {
    // Because this struct has at least one private field, whilst it itself is pub(lic), it cannot
    //   be initialised by anything outside of this module
    // The only way to create a Graph object, is using the constructor defined below
    pub num_vertices: usize,
    adjacency_lists: Vec<Vec<DigraphEdge>>,
}

#[derive(Clone, Debug)]
pub struct DigraphEdge {
    pub to: usize,
    pub weight: f32,
}

impl Digraph {
    pub fn new(num_vertices: usize) -> Self {
        let mut out = Self {
            num_vertices,
            adjacency_lists: vec![],
        };
        for _ in 0..num_vertices {
            out.adjacency_lists.push(vec![]);
        }
        out
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f32) {
        let e = DigraphEdge{to, weight};
        self.adjacency_lists[from].push(e);
    }

    pub fn adj(&self, node_number: usize) -> &Vec<DigraphEdge> {
        &self.adjacency_lists[node_number]
    }
}
