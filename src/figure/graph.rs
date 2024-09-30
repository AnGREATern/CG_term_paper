use super::vertex::Vertex;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader, ErrorKind},
};

pub struct Graph {
    pub vertexes: Vec<Vertex>,
    faces: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertexes: Vec<Vertex>, faces: Vec<Vec<usize>>) -> Self {
        Self { vertexes, faces }
    }

    pub fn load(filename: &str) -> io::Result<Self> {
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
            if vals.len() != 4 {
                return Err(ErrorKind::InvalidData.into());
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

        Ok(Self::new(vertexes, faces))
    }

    // pub fn draw(&self) {
    //     for face in &self.faces {

    //     }
    // }

    // pub fn add_edge(&mut self, from: usize, to: usize) {
    //     self.edges[from].push(to);
    //     if !self.is_directed {
    //         self.edges[to].push(from);
    //     }
    // }
}
