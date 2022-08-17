#[derive(Debug)]
pub struct Graph {
    num_vertices: i32,
    adjacency_lists: Vec<Vec<i32>>
}

impl Graph {
    pub fn new(num_vertices: i32) -> Graph {
        let mut out = Graph {num_vertices, adjacency_lists: vec![] };
        for _ in 0..num_vertices {
            out.adjacency_lists.push(vec![]);
        }
        return out;
    }

    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.adjacency_lists[from as usize].push(to);
        self.adjacency_lists[to as usize].push(from);
    }
}

