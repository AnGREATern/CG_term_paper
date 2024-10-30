use super::edge::Edge;
use super::object::Object;
use super::vertex::Vertex;

pub struct Projection {
    center: Vertex,
    radius: f64,
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Projection {
    pub fn new(object: &Object, radius: f64) -> Self {
        let center = object.center();
        let nfaces = object.nfaces();
        let mut vertexes = Vec::with_capacity(nfaces);
        // 3 - number of vertexes in face
        let mut edges = Vec::with_capacity(3 * nfaces);

        for face_ind in 0..nfaces {
            let face = object.face(face_ind);
            for v in face.into_iter() {
                vertexes.push(v.project_to_sphere(center, radius));
            }
            edges.push(Edge::new(face_ind, face_ind + 1));
            edges.push(Edge::new(face_ind + 1, face_ind + 2));
            edges.push(Edge::new(face_ind, face_ind + 2));
        }

        Self {
            center,
            radius,
            vertexes,
            edges,
        }
    }
}
