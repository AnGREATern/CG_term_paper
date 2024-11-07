use super::edge::Edge;
use std::collections::{btree_set::Iter, BTreeSet};

#[derive(Clone)]
pub struct EdgeSet(BTreeSet<Edge>);

impl EdgeSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn add(&mut self, from: usize, to: usize) -> bool {
        if from < to {
            self.0.insert(Edge::new(from, to))
        } else if from > to {
            self.0.insert(Edge::new(to, from))
        } else {
            false
        }
    }

    pub fn insert(&mut self, edge: &Edge) -> bool {
        if let Some(edge) = edge.norm() {
            self.0.insert(edge)
        } else {
            false
        }
    }

    pub fn iter(&self) -> Iter<'_, Edge> {
        self.0.iter()
    }

    pub fn remove(&mut self, edge: &Edge) -> bool {
        if let Some(edge) = edge.norm() {
            self.0.remove(&edge)
        } else {
            false
        }
    }
}
