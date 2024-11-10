use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    pub fn norm(&self) -> Option<Edge> {
        let from = self.from;
        let to = self.to;
        match from.cmp(&to) {
            Ordering::Less => Some(Edge::new(from, to)),
            Ordering::Greater => Some(Edge::new(to, from)),
            Ordering::Equal => None,
        }
    }
}
