#[derive(Debug)]
pub struct Graph {
    // Because this struct has at least one private field, whilst it itself is pub(lic), it cannot
    //   be initialised by anything outside of this module
    // The only way to create a Graph object, is using the constructor defined below
    num_vertices: i32,
    adjacency_lists: Vec<Vec<i32>>,
}

impl Graph {
    pub fn new(num_vertices: i32) -> Graph {
        let mut out = Graph {
            num_vertices,
            adjacency_lists: vec![],
        };
        for _ in 0..num_vertices {
            out.adjacency_lists.push(vec![]);
        }
        out
    }

    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.adjacency_lists[from as usize].push(to);
        self.adjacency_lists[to as usize].push(from);
    }

    fn adj(&self, node_number: i32) -> Vec<i32> {
        self.adjacency_lists[node_number as usize].clone()
    }

    pub fn connected_components(&mut self, out: &mut Vec<Vec<i32>>) {
        let mut nodes_visited: Vec<bool> = vec![false; self.num_vertices as usize];
        let mut node_component_numbers: Vec<i32> = vec![-1; self.num_vertices as usize];
        let mut current_component_number = 0;
        for v in 0..self.num_vertices {
            if !nodes_visited[v as usize] {
                self._dfs(&mut nodes_visited, &mut node_component_numbers, current_component_number, v);
                current_component_number += 1;
            }
        }

        for i in 0..current_component_number {
            let mut component_nodes = vec![];
            for j in 0..node_component_numbers.len() {
                if i == node_component_numbers[j as usize] {
                    component_nodes.push(j as i32);
                }
            }
            out.push(component_nodes);
        }
    }

    fn _dfs(&mut self, nodes_visited: &mut Vec<bool>, node_component_numbers: &mut Vec<i32>, current_component_number: i32, node_to_visit: i32) {
        nodes_visited[node_to_visit as usize] = true;
        node_component_numbers[node_to_visit as usize] = current_component_number;
        for w in self.adj(node_to_visit) {
            if !nodes_visited[w as usize] {
                self._dfs(nodes_visited, node_component_numbers, current_component_number, w);
            }
        }
    }
}
