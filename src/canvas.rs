use std::{f64::MIN, mem::swap};

use crate::color::Color;
use crate::figure::{object::Object, vertex::Vertex};

pub struct Canvas {
    frame: Vec<u8>,
    width: u32,
    height: u32,
    color: Color,
    zbuffer: Vec<f64>,
}

impl Canvas {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        let frame = vec![0; color.len() * (width * height) as usize];
        let zbuffer = vec![MIN; (height * width) as usize];
        let mut res = Self {
            frame,
            width,
            height,
            color,
            zbuffer,
        };
        res.fill();

        res
    }

    pub fn frame(&self) -> &[u8] {
        &self.frame.as_slice()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn clear(&mut self) {
        self.fill();
        self.zbuffer = vec![MIN; (self.height * self.width) as usize];
    }

    pub fn draw_object(&mut self, object: &Object, mut light_direction: Vertex) {
        light_direction.normalize();
        let color = object.color();
        for face_ind in 0..object.nfaces() {
            let world_coords = object.face(face_ind);
            let mut n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);
            n.normalize();
            let intensity = light_direction * n;
            let screen_coords = vec![
                world_coords[0].world_to_screen(self.height, self.width),
                world_coords[1].world_to_screen(self.height, self.width),
                world_coords[2].world_to_screen(self.height, self.width),
            ];
            let mut cur_color = color.clone();
            if intensity > 0. {
                cur_color.set_rgb(
                    (cur_color.r() as f64 * intensity) as u8,
                    (cur_color.g() as f64 * intensity) as u8,
                    (cur_color.b() as f64 * intensity) as u8,
                );
            } else {
                cur_color = self.color.clone();
            }

            self.draw_triangle(screen_coords, cur_color);
        }
    }
}

impl Canvas {
    fn fill(&mut self) {
        for chunk in self.frame.chunks_exact_mut(self.color.len()) {
            chunk.copy_from_slice(&self.color.to_array());
        }
    }

    fn draw_triangle(&mut self, mut coords: Vec<Vertex>, color: Color) {
        if coords[0].y == coords[1].y && coords[1].y == coords[2].y {
            return;
        }

        coords.sort_by(|a, b| a.y.partial_cmp(&b.y).expect("draw_triangle: sorting"));
        let total_height = (coords[2].y - coords[0].y) as i32;
        for i in 0..total_height {
            let is_second_half =
                (i > (coords[1].y - coords[0].y) as i32) || (coords[1].y == coords[0].y);
            let segment_height = if is_second_half {
                coords[2].y - coords[1].y
            } else {
                coords[1].y - coords[0].y
            };
            let alpha = i as f64 / total_height as f64;
            let beta = if is_second_half {
                (i as f64 - coords[1].y + coords[0].y) / segment_height
            } else {
                i as f64 / segment_height
            };
            let mut a = coords[0] + (coords[2] - coords[0]) * alpha;
            let mut b = if is_second_half {
                coords[1] + (coords[2] - coords[1]) * beta
            } else {
                coords[0] + (coords[1] - coords[0]) * beta
            };
            a.round();
            b.round();
            if a.x > b.x {
                swap(&mut a, &mut b);
            }
            for j in a.x as i32..=b.x as i32 {
                let phi = if a.x == b.x {
                    1.
                } else {
                    (j as f64 - a.x) / (b.x - a.x)
                };
                let mut p = a + (b - a) * phi;
                p.round();
                let idx = (p.x as u32 + p.y as u32 * self.width) as usize;
                if idx < self.zbuffer.len() && self.zbuffer[idx] < p.z {
                    self.zbuffer[idx] = p.z;
                    self.set_pixel(p.x.round() as u32, p.y.round() as u32, color.clone());
                }
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let pixel = color.len() * (x + y * self.width) as usize;
        self.frame[pixel..pixel + color.len()].copy_from_slice(&color.to_array());
    }
}
