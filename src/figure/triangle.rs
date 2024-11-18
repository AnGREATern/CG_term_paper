use super::vertex::Vertex;
use crate::EPS;

pub struct Triangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Self {
        Triangle { a, b, c }
    }

    pub fn normal(&self) -> Vertex {
        (self.b - self.a) ^ (self.c - self.a)
    }

    pub fn contains(&self, v: Vertex) -> bool {
        let a = self.a - v;
        let b = self.b - v;
        let c = self.c - v;
        let area = self.normal().len();
        let sum = (a ^ b).len() + (b ^ c).len() + (c ^ a).len();

        (area - sum).abs() < EPS
    }

    pub fn intersect(&self, a: Vertex, b: Vertex) -> Option<Vertex> {
        let normal = self.normal();
        let div = (a - b) * normal;
        if div.abs() < EPS {
            return None;
        }
        let t = (a - self.a) * normal / div;
        let v = a + (b - a) * t;

        if t > -EPS && self.contains(v) {
            Some(v)
        } else {
            None
        }
    }
}
