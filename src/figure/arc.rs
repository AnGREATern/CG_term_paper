use super::vertex::Vertex;
use crate::consts::EPS;

pub struct Arc {
    a: Vertex,
    b: Vertex,
    a_id: usize,
    b_id: usize,
}

pub enum ArcIntersectionResult {
    I((usize, f64), (usize, f64)), // `I` shape, two arcs are co-planar
    T1(usize, f64),                // `T` shape 1, A's endpoint on B, return A's endpoint id
    T2(usize, f64),                // `T` shape 2, B's endpoint on A, return B's endpoint id
    L(usize, usize), // `L` shape, A's endpoint on B's endpoint, return two endpoint ids
    X(Vertex, f64),  // `X` shape, return the intersection coordinates
    N,               // no intersection
    S,               // Two arcs are same
}

impl Arc {
    pub fn new(a: Vertex, b: Vertex, a_id: usize, b_id: usize) -> Self {
        Self { a, b, a_id, b_id }
    }
}

impl Arc {
    fn planar_contains(&self, v: Vertex) -> bool {
        let ab = self.a ^ self.b;
        let ba = -ab;

        ((self.a ^ v) * ab) > EPS && ((self.b ^ v) * ba) > EPS
    }

    fn contains(&self, mut v: Vertex) -> Option<f64> {
        if (v * (self.a ^ self.b)).abs() < EPS && self.planar_contains(v) {
            let mut a = self.a;
            let mut b = self.b;
            a.normalize();
            b.normalize();
            v.normalize();
            Some((v * a).acos() / (b * a).acos())
        } else {
            None
        }
    }
}

impl Arc {
    pub fn intersect(a: &Arc, b: &Arc) -> ArcIntersectionResult {
        let ab = a.a ^ a.b;
        let ba = -ab;
        let cd = b.a ^ b.b;
        let dc = -cd;

        if (ab ^ cd).len() < EPS {
            if (a.a * cd).abs() > EPS {
                return ArcIntersectionResult::N;
            }

            let mut res = Vec::new();
            for (v, id) in vec![(a.a, a.a_id), (a.b, a.b_id)] {
                if v == b.a {
                    res.push((id, 0.))
                } else if v == b.b {
                    res.push((id, 1.))
                } else if let Some(k) = b.contains(v) {
                    res.push((id, k));
                } else if (v - b.a).len2() < (v - b.b).len2() {
                    res.push((id, -1.));
                } else {
                    res.push((id, 2.));
                }
            }

            if res.len() != 2 {
                return ArcIntersectionResult::N;
            }
            if res[0].1 > res[1].1 {
                res.swap(0, 1);
            }
            if (res[0].1 < 0. && res[1].1 < 0.) || (res[0].1 > 1. && res[1].1 > 1.) {
                return ArcIntersectionResult::N;
            }
            if (res[0].1 < 0. && a.planar_contains(b.a))
                || (res[1].1 > 1. && !a.planar_contains(b.b)) {
                return ArcIntersectionResult::N;
            }
            if res[0].1 == 0. && res[1].1 == 1. {
                return ArcIntersectionResult::S;
            }
            if res[0].1 < 0. && res[1].1 == 0. {
                return ArcIntersectionResult::L(res[1].0, b.a_id);
            }
            if res[0].1 == 1. && res[1].1 > 1. {
                return ArcIntersectionResult::L(res[0].0, b.b_id);
            }
            return ArcIntersectionResult::I(res[0], res[1]);
        }

        if a.a == b.a {
            return ArcIntersectionResult::L(a.a_id, b.a_id);
        } else if a.a == b.b {
            return ArcIntersectionResult::L(a.a_id, b.b_id);
        } else if a.b == b.a {
            return ArcIntersectionResult::L(a.b_id, b.a_id);
        } else if a.b == b.b {
            return ArcIntersectionResult::L(a.b_id, b.b_id);
        }

        if let Some(k) = b.contains(a.a) {
            return ArcIntersectionResult::T1(a.a_id, k);
        } else if let Some(k) = b.contains(a.b) {
            return ArcIntersectionResult::T1(a.b_id, k);
        } else if let Some(k) = a.contains(b.a) {
            return ArcIntersectionResult::T2(b.a_id, k);
        } else if let Some(k) = a.contains(b.b) {
            return ArcIntersectionResult::T2(b.b_id, k);
        }

        if (b.a * ab) * (b.b * ba) > EPS && (a.a * cd) * (a.b * dc) > EPS {
            let div = (a.a - a.b) * cd;
            if div.abs() < EPS {
                return ArcIntersectionResult::N;
            }
            let t = (a.a * cd) / div;
            let mut v_unit = a.a + (a.b - a.a) * t;
            v_unit.normalize();

            let c_len = b.a.len();
            let c_unit = b.a / c_len;
            let mut d_unit = b.b;
            d_unit.normalize();
            let v = v_unit * c_len;

            if b.planar_contains(v) {
                let k = (v_unit * c_unit).acos() / (d_unit * c_unit).acos();
                return ArcIntersectionResult::X(v, k);
            } else {
                return ArcIntersectionResult::N;
            }
        }

        ArcIntersectionResult::N
    }
}
