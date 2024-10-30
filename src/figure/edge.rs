pub struct Edge {
    pub from: usize,
    pub to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}
