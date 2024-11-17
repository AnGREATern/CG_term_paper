use crate::{color::Color, EPS};

use super::{
    arc::{Arc, ArcIntersectionResult},
    edge::Edge,
    edge_set::EdgeSet,
    graph::{Graph, RcGraphEdge},
    object::Object,
    projection::Projection,
    vertex::Vertex,
};

use std::cmp::Ordering;
use std::f64::consts::PI;

pub struct MergedObject {
    faces: Vec<Vec<usize>>,
    vertexes_pairs: Vec<(Vertex, Vertex)>,
    color_pairs: (Color, Color),
}

struct SphereVertex {
    vertex: Vertex,
    index: usize,
    origin_id: usize,
}

impl MergedObject {
    pub fn new(src_proj: Projection, dst_proj: Projection) -> Self {
        let n = src_proj.nvertexes();
        let m = dst_proj.nvertexes();
        let mut sphere_vertexes = Vec::with_capacity(n + m);
        let mut edges = EdgeSet::new();

        for i in 0..n {
            sphere_vertexes.push(SphereVertex {
                vertex: src_proj.sphere_vertex(i),
                index: i,
                origin_id: 1,
            });
        }
        for i in 0..m {
            sphere_vertexes.push(SphereVertex {
                vertex: dst_proj.sphere_vertex(i),
                index: i,
                origin_id: 2,
            });
        }

        for src_edge in src_proj.edges_iter() {
            edges.insert(src_edge);
        }
        for dst_edge in dst_proj.edges_iter() {
            let dst_edge = Edge::new(dst_edge.from + n, dst_edge.to + n);
            let v1 = sphere_vertexes[dst_edge.from].vertex;
            let v2 = sphere_vertexes[dst_edge.to].vertex;
            let dst_arc = Arc::new(v1, v2, dst_edge.from, dst_edge.to);
            let mut intersections = vec![(0., dst_edge.from), (1., dst_edge.to)];
            let mut is_skip_add = false;
            for src_edge in edges.clone().iter() {
                let u1 = sphere_vertexes[src_edge.from].vertex;
                let u2 = sphere_vertexes[src_edge.to].vertex;
                let src_arc = Arc::new(u1, u2, src_edge.from, src_edge.to);
                match Arc::intersect(&src_arc, &dst_arc) {
                    ArcIntersectionResult::T1(index, k) => intersections.push((k, index)),
                    ArcIntersectionResult::T2(index, ..) => {
                        edges.remove(src_edge);
                        edges.add(src_edge.from, index);
                        edges.add(src_edge.to, index);
                    }
                    ArcIntersectionResult::X(vertex, k) => {
                        let id = sphere_vertexes.len();
                        sphere_vertexes.push(SphereVertex {
                            vertex,
                            origin_id: 0,
                            index: 0,
                        });
                        edges.remove(src_edge);
                        edges.add(src_edge.from, id);
                        edges.add(src_edge.to, id);
                        intersections.push((k, id));
                    }
                    ArcIntersectionResult::I((id1, k1), (id2, k2)) => {
                        edges.remove(src_edge);
                        if k1 > 0. {
                            intersections.push((k1, id1));
                        } else if k1 < 0. {
                            edges.add(id1, intersections[0].1);
                        }

                        if k2 < 1. {
                            intersections.push((k2, id2));
                        } else if k2 > 1. {
                            edges.add(id2, intersections[1].1);
                        }
                    }
                    ArcIntersectionResult::L(id1, id2) => {
                        if id2 == dst_edge.from {
                            intersections[0].1 = id1;
                        } else if id2 == dst_edge.to {
                            intersections[1].1 = id1;
                        }
                    }
                    ArcIntersectionResult::S => {
                        is_skip_add = true;
                        break;
                    }
                    ArcIntersectionResult::N => {}
                }
            }
            if is_skip_add {
                continue;
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            for i in 0..intersections.len() - 1 {
                edges.add(intersections[i].1, intersections[i + 1].1);
            }
        }

        let mut vertexes_pairs = Vec::new();
        for vertex in sphere_vertexes.iter() {
            let mut p = match vertex.origin_id {
                1 => (
                    src_proj.vertex(vertex.index),
                    dst_proj.project_from_sphere(vertex.vertex),
                ),
                2 => (
                    src_proj.project_from_sphere(vertex.vertex),
                    dst_proj.vertex(vertex.index),
                ),
                _ => (
                    src_proj.project_from_sphere(vertex.vertex),
                    dst_proj.project_from_sphere(vertex.vertex),
                ),
            };
            p.0 -= *src_proj.center();
            p.1 -= *dst_proj.center();
            vertexes_pairs.push(p);
        }
        // let bbox1 = Vertex::bounding_box(&vertexes_pairs.iter().map(|p| p.0).collect());
        // let bbox2 = Vertex::bounding_box(&vertexes_pairs.iter().map(|p| p.1).collect());
        // let scale1 = (bbox1.1 - bbox1.0).max();
        // let scale2 = (bbox2.1 - bbox2.0).max();

        // let scale = scale1.max(scale2);
        // for p in vertexes_pairs.iter_mut() {
        //     p.0 /= scale;
        //     p.1 /= scale;
        // }
        // for p in vertexes_pairs.iter_mut() {
        //     p.0 /= scale1;
        //     p.1 /= scale2;
        // }

        let sphere_vertexes = sphere_vertexes.iter().map(|v| v.vertex).collect();
        let faces = Self::resolve_faces(&sphere_vertexes, &edges);

        let mut triangle_faces = Vec::new();
        let mut set = std::collections::BTreeSet::<Vec<usize>>::new();
        for f in faces.into_iter() {
            if f.len() > 3 {
                for i in 1..f.len() - 1 {
                    let mut tri = vec![f[0], f[i], f[i + 1]];
                    tri.sort();
                    if f[0] == f[i] || f[0] == f[i + 1] {
                        continue;
                    }
                    if set.insert(tri.clone()) {
                        Self::adjust_order(&mut tri, &sphere_vertexes, Vertex::new(0.0, 0.0, 0.0));
                        triangle_faces.push(tri);
                    }
                }
            } else {
                let mut tri = f;
                tri.sort();
                if set.insert(tri.clone()) {
                    Self::adjust_order(&mut tri, &sphere_vertexes, Vertex::new(0.0, 0.0, 0.0));
                    triangle_faces.push(tri);
                }
            }
        }

        Self {
            vertexes_pairs,
            faces: triangle_faces,
            color_pairs: (src_proj.color().clone(), dst_proj.color().clone()),
        }
    }

    pub fn interpolation(&self, ratio: f64) -> Object {
        let mut new_verts = Vec::new();
        for (v1, v2) in self.vertexes_pairs.iter() {
            new_verts.push(*v1 + (*v2 - *v1) * ratio);
        }
        let color = Color::join_by_part(
            self.color_pairs.0.clone(),
            self.color_pairs.1.clone(),
            ratio,
        );

        Object::new(new_verts, self.faces.clone(), color)
    }
}

impl MergedObject {
    fn resolve_faces(verts: &Vec<Vertex>, edges: &EdgeSet) -> Vec<Vec<usize>> {
        let n = verts.len();
        let mut graph = Graph::new(verts);
        for e in edges.iter() {
            graph.add_pair(e.from, e.to);
        }

        for i in 0..n {
            let v = verts[i];
            let v_len = v.len();
            let m = graph.neighbors_count(i);
            if m < 1 {
                continue;
            }
            let first = verts[graph.neighbors(i).next().unwrap().borrow().to];
            let mut first_dir = first - v * (v * first / v_len);
            first_dir.normalize();
            let mut adj_edges = graph
                .neighbors(i)
                .map(|e| {
                    let p = verts[e.borrow().to];
                    let mut dir = p - v * (v * p / v_len);
                    dir.normalize();
                    let norm = first_dir ^ dir;
                    let cos = first_dir * dir;
                    let mut angle = if (cos - 1.).abs() < EPS {
                        0.
                    } else if (cos + 1.).abs() < EPS {
                        PI
                    } else {
                        cos.acos()
                    };
                    if v * norm < -EPS {
                        angle = -angle;
                    }
                    (angle, e)
                })
                .collect::<Vec<(f64, &RcGraphEdge)>>();
            adj_edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
            for j in 0..m {
                let k = if j == m - 1 { 0 } else { j + 1 };
                adj_edges[j].1.borrow_mut().next = std::rc::Rc::downgrade(&adj_edges[k].1);
            }
        }

        let mut faces = Vec::new();
        for i in 0..n {
            for e in graph.neighbors(i) {
                let mut e = e.clone();
                let mut one_face = Vec::new();
                while !e.borrow().visited {
                    let p = e.borrow().to;
                    one_face.push(p);
                    e.borrow_mut().visited = true;
                    let o = e.borrow().oppo.upgrade().expect("No opposite edge!");
                    let n = o.borrow().next.upgrade().expect("No next edge");
                    e = n;
                }
                if one_face.len() > 2 {
                    faces.push(one_face);
                }
            }
        }

        faces
    }

    fn adjust_order(face: &mut Vec<usize>, verts: &Vec<Vertex>, center: Vertex) {
        if !Self::check_order(face, verts, center) {
            face.reverse();
        }
    }

    fn check_order(face: &Vec<usize>, verts: &Vec<Vertex>, center: Vertex) -> bool {
        let n = face.len();
        if n < 3 {
            return false;
        }
        let face = face.iter().map(|id| verts[*id]).collect::<Vec<_>>();

        Vertex::triple_prod(face[0] - center, face[1] - center, face[2] - center) > EPS
    }
}
