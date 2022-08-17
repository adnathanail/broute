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

    fn adj(&self, node_number: i32) -> Vec<i32> {
        return self.adjacency_lists[node_number as usize].clone();
    }

    pub fn dfs(&mut self) {
        let mut marked: Vec<bool> = vec![false; self.num_vertices as usize];
        let mut edge_to: Vec<i32> = vec![-1; self.num_vertices as usize];
        self._dfs(&mut marked, &mut edge_to, 0);
        println!("{:?}", marked);
        println!("{:?}", edge_to);
    }

    fn _dfs(&mut self, marked: &mut Vec<bool>, edge_to: &mut Vec<i32>, node_to_visit: i32) {
        marked[node_to_visit as usize] = true;
        for w in self.adj(node_to_visit) {
            if !marked[w as usize] {
                self._dfs(marked, edge_to, w);
                edge_to[w as usize] = node_to_visit;
            }
        }
    }
}

