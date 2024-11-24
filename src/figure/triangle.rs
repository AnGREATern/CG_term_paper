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

    pub fn normal_inside(&self, v: Vertex, normals: Vec<Vertex>) -> Vertex {
        let av = v - self.a;
        let bv = v - self.b;
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let bc = self.c - self.b;

        let s_abv = (av ^ ab).len() / 2.;
        let s_bcv = (bv ^ bc).len() / 2.;
        let s_acv = (av ^ ac).len() / 2.;
        let s_abc = (ab ^ ac).len() / 2.;

        let t1 = s_bcv / s_abc;
        let t2 = s_acv / s_abc;
        let t3 = s_abv / s_abc;

        normals[0] * t1 + normals[1] * t2 + normals[2] * t3
    }
}
