use std::fmt;

#[derive(Debug)]
pub struct Digraph {
    // Because this struct has at least one private field, whilst it itself is pub(lic), it cannot
    //   be initialised by anything outside of this module
    // The only way to create a Graph object, is using the constructor defined below
    pub num_vertices: usize,
    adjacency_lists: Vec<Vec<DigraphEdge>>,
}

impl fmt::Display for Digraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: `f`
        // Returns `fmt::Result` which indicates whether the operation succeeded or failed
        writeln!(f, "{} nodes", self.num_vertices)?;
        self.adjacency_lists.iter().enumerate().fold(
            Ok(()),
            |result, (from_node, adjacency_list)| {
                result.and_then(|_| {
                    writeln!(f, "\t{}", from_node)?;
                    adjacency_list.iter().fold(Ok(()), |result, edge| {
                        result.and_then(|_| writeln!(f, "\t\t{}", edge))
                    })
                })
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct DigraphEdge {
    pub to: usize,
    pub weight: f32,
}

impl fmt::Display for DigraphEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "to {} (weight {})", self.to, self.weight)
    }
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
        let e = DigraphEdge { to, weight };
        self.adjacency_lists[from].push(e);
    }

    pub fn adj(&self, node_number: usize) -> &Vec<DigraphEdge> {
        &self.adjacency_lists[node_number]
    }
}
