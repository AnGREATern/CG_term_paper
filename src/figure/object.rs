use crate::color::Color;

use super::vertex::Vertex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    slice::Iter,
};

#[derive(Clone)]
pub struct Object {
    vertexes: Vec<Vertex>,
    faces: Vec<Vec<usize>>,
    center: Vertex,
    color: Color,
}

impl Object {
    pub fn new(vertexes: Vec<Vertex>, faces: Vec<Vec<usize>>, color: Color) -> Self {
        let center = Vertex::center(&vertexes);
        Self {
            vertexes,
            faces,
            center,
            color,
        }
    }

    pub fn load(filename: &str, color: Color) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut vertexes = Vec::new();
        let mut faces = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let vals = line.split_whitespace().collect::<Vec<_>>();
            if vals.len() == 0 {
                continue;
            }
            match vals[0] {
                "v" => vertexes.push(Vertex::new(
                    vals[1].parse().unwrap(),
                    vals[2].parse().unwrap(),
                    vals[3].parse().unwrap(),
                )),
                "f" => faces.push(vec![
                    vals[1].split("/").next().unwrap().parse::<usize>().unwrap() - 1,
                    vals[2].split("/").next().unwrap().parse::<usize>().unwrap() - 1,
                    vals[3].split("/").next().unwrap().parse::<usize>().unwrap() - 1,
                ]),
                _ => {}
            }
        }

        Ok(Self::new(vertexes, faces, color))
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn center(&self) -> &Vertex {
        &self.center
    }

    pub fn face(&self, index: usize) -> Vec<Vertex> {
        let mut res = vec![];
        for nv in self.faces[index].iter() {
            res.push(self.vertexes[*nv]);
        }

        res
    }

    pub fn face_indexes(&self, index: usize) -> Vec<usize> {
        self.faces[index].clone()
    }

    pub fn nvertexes(&self) -> usize {
        self.vertexes.len()
    }

    pub fn vertex(&self, index: usize) -> Vertex {
        self.vertexes[index]
    }

    pub fn vertexes_iter(&self) -> Iter<'_, Vertex> {
        self.vertexes.iter()
    }

    pub fn mov(&mut self, delta: Vertex) {
        self.center.mov(delta);
        for vertex in self.vertexes.iter_mut() {
            vertex.mov(delta);
        }
    }

    pub fn scale(&mut self, k: f64) {
        for vertex in self.vertexes.iter_mut() {
            vertex.scale(&self.center, k);
        }
    }

    pub fn rotate(&mut self, angles: Vertex) {
        let (sin_x, cos_x) = angles.x.sin_cos();
        let (sin_y, cos_y) = angles.y.sin_cos();
        let (sin_z, cos_z) = angles.z.sin_cos();
        for vertex in self.vertexes.iter_mut() {
            let y1 = vertex.y * cos_x - vertex.z * sin_x;
            let z1 = vertex.y * sin_x + vertex.z * cos_x;

            let x2 = vertex.x * cos_y + z1 * sin_y;
            let z2 = -vertex.x * sin_y + z1 * cos_y;

            let x3 = x2 * cos_z - y1 * sin_z;
            let y3 = x2 * sin_z + y1 * cos_z;

            vertex.x = x3;
            vertex.y = y3;
            vertex.z = z2;
            // vertex.rotate(angles);
        }
    }
}
