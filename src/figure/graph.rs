use super::vertex::Vertex;

// Adjacency list
pub struct Graph {
    vertexes: Vec<Vertex>,
    edges: Vec<Vec<usize>>,
    is_directed: bool,
}

impl Graph {
    pub fn new(vertexes: Vec<Vertex>, is_directed: Option<bool>) -> Self {
        let edges = vec![vec![]; vertexes.len()];
        let is_directed = is_directed.unwrap_or(false);
        Self {
            vertexes,
            edges,
            is_directed,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
        if !self.is_directed {
            self.edges[to].push(from);
        }
    }
}
