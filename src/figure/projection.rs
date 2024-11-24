use std::collections::btree_set::Iter;

use crate::color::Color;

use super::{edge::Edge, edge_set::EdgeSet, object::Object, triangle::Triangle, vertex::Vertex};

pub struct Projection {
    radius: f64,
    sphere_vertexes: Vec<Vertex>,
    edges: EdgeSet,
    vn: Vec<usize>,
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
        let mut vn = vec![0; object.nvertexes()];
        for face_ind in 0..nfaces {
            let face = object.face_indexes(face_ind);
            edges.add(face[0].0, face[1].0);
            edges.add(face[0].0, face[2].0);
            edges.add(face[1].0, face[2].0);

            vn[face[0].0] = face[0].1;
            vn[face[1].0] = face[1].1;
            vn[face[2].0] = face[2].1;
        }

        Self {
            radius,
            sphere_vertexes,
            vn,
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

    pub fn normal(&self, index: usize) -> Vertex {
        self.object.normal(self.vn[index])
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

    pub fn project_from_sphere(&self, v: Vertex) -> Result<(Vertex, Vertex), ()> {
        // vertex and normal
        let center = *self.object.center();
        for ind in 0..self.object.nfaces() {
            let coords = self.object.face_coords(ind);
            let tri = Triangle::new(coords[0], coords[1], coords[2]);
            if let Some(int) = tri.intersect(center, center + v) {
                let normal = tri.normal_inside(int, self.object.face_normals(ind));
                return Ok((int, normal));
            }
        }

        Err(())
    }
}
