use super::edge_set::EdgeSet;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

pub type RcGraphEdge = Rc<RefCell<GraphEdge>>;
pub type WeakGraphEdge = Weak<RefCell<GraphEdge>>;

pub struct Graph {
    edges: Vec<Vec<RcGraphEdge>>,
    index_map: Vec<usize>,
    nr_nodes: usize,
    unique_edges: EdgeSet,
}

impl Graph {
    pub fn new<T: Ord>(nodes: &Vec<T>) -> Self {
        let n = nodes.len();
        let mut index_map = Vec::with_capacity(n);
        let mut map = BTreeMap::<&T, usize>::new();

        for i in 0..n {
            let v = &nodes[i];
            let id = if let Some(&id) = map.get(&v) { id } else { i };
            map.insert(v, id);
            index_map.push(id);
        }

        Self {
            edges: vec![Vec::new(); n],
            index_map,
            nr_nodes: n,
            unique_edges: EdgeSet::new(),
        }
    }

    pub fn add_pair(&mut self, from: usize, to: usize) {
        let (from, to) = (self.index_map[from], self.index_map[to]);
        if !self.unique_edges.add(from, to) {
            return;
        }
        let e1 = Rc::new(RefCell::new(GraphEdge::new(from, to)));
        let e2 = Rc::new(RefCell::new(GraphEdge::new(to, from)));
        e1.borrow_mut().oppo = Rc::downgrade(&e2);
        e2.borrow_mut().oppo = Rc::downgrade(&e1);
        self.edges[from].push(e1);
        self.edges[to].push(e2);
    }

    pub fn neighbors_count(&self, index: usize) -> usize {
        self.edges[index].len()
    }

    pub fn neighbors(&self, index: usize) -> impl Iterator<Item = &RcGraphEdge> {
        self.edges[index].iter()
    }
}

pub struct GraphEdge {
    pub from: usize,
    pub to: usize,
    pub oppo: WeakGraphEdge,
    pub next: WeakGraphEdge,
    pub visited: bool,
}

impl GraphEdge {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            oppo: Weak::new(),
            next: Weak::new(),
            visited: false,
        }
    }
}
