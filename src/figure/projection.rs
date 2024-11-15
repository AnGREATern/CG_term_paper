use std::collections::btree_set::Iter;

use crate::color::Color;

use super::{edge::Edge, edge_set::EdgeSet, object::Object, triangle::Triangle, vertex::Vertex};

pub struct Projection {
    radius: f64,
    sphere_vertexes: Vec<Vertex>,
    edges: EdgeSet,
    object: Object,
}

impl Projection {
    pub fn new(object: Object, radius: f64) -> Self {
        let center = *object.center();
        let nfaces = object.nfaces();
        let mut sphere_vertexes = vec![];
        let mut edges = EdgeSet::new();
        for vertex in object.vertexes_iter() {
            sphere_vertexes.push(vertex.project_to_sphere(center, radius));
        }
        for face_ind in 0..nfaces {
            let face = object.face_indexes(face_ind);
            edges.add(face[0], face[1]);
            edges.add(face[0], face[2]);
            edges.add(face[1], face[2]);
        }

        Self {
            radius,
            sphere_vertexes,
            edges,
            object,
        }
    }

    pub fn color(&self) -> Color {
        self.object.color()
    }

    pub fn sphere_vertex(&self, index: usize) -> Vertex {
        self.sphere_vertexes[index]
    }

    pub fn vertex(&self, index: usize) -> Vertex {
        self.object.vertex(index)
    }

    pub fn nvertexes(&self) -> usize {
        self.object.nvertexes()
    }

    pub fn edges_iter(&self) -> Iter<'_, Edge> {
        self.edges.iter()
    }

    pub fn center(&self) -> &Vertex {
        self.object.center()
    }

    pub fn project_from_sphere(&self, v: Vertex) -> Vertex {
        let center = *self.object.center();
        for ind in 0..self.object.nfaces() {
            let f = self.object.face(ind);
            let tri = Triangle::new(f[0], f[1], f[2]);
            if let Some(int) = tri.intersect(center, center + v) {
                return int;
            }
        }
        panic!("No intersect found! {}, {}, {}", v.x, v.y, v.z);
    }
}
