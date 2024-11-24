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
        for face_ind in 0..object.nfaces() {
            let world_coords = object.face_coords(face_ind);
            let mut intensities = vec![];
            for normal in object.face_normals(face_ind) {
                intensities.push(light_direction * normal);
            }
            let mut screen_coords = vec![];
            for i in 0..3 {
                screen_coords.push((
                    world_coords[i].world_to_screen(self.height, self.width),
                    intensities[i],
                ));
            }
            self.draw_triangle(screen_coords, object.color());
        }
    }
}

impl Canvas {
    fn fill(&mut self) {
        for chunk in self.frame.chunks_exact_mut(self.color.len()) {
            chunk.copy_from_slice(&self.color.to_array());
        }
    }

    fn draw_triangle(&mut self, mut coords: Vec<(Vertex, f64)>, color: Color) {
        if coords[0].0.y == coords[1].0.y && coords[1].0.y == coords[2].0.y {
            return;
        }

        coords.sort_by(|a, b| a.0.y.partial_cmp(&b.0.y).expect("draw_triangle: sorting"));
        let total_height = (coords[2].0.y - coords[0].0.y) as i32;
        for i in 0..total_height {
            let is_second_half =
                (i > (coords[1].0.y - coords[0].0.y) as i32) || (coords[1].0.y == coords[0].0.y);
            let segment_height = if is_second_half {
                coords[2].0.y - coords[1].0.y
            } else {
                coords[1].0.y - coords[0].0.y
            };
            let alpha = i as f64 / total_height as f64;
            let mut a_side_intensity = coords[0].1 + (coords[2].1 - coords[0].1) * alpha;
            let beta = if is_second_half {
                (i as f64 - coords[1].0.y + coords[0].0.y) / segment_height
            } else {
                i as f64 / segment_height
            };
            let mut a = coords[0].0 + (coords[2].0 - coords[0].0) * alpha;
            let mut b = if is_second_half {
                coords[1].0 + (coords[2].0 - coords[1].0) * beta
            } else {
                coords[0].0 + (coords[1].0 - coords[0].0) * beta
            };
            let mut b_side_intensity = if is_second_half {
                coords[1].1 + (coords[2].1 - coords[1].1) * beta
            } else {
                coords[0].1 + (coords[1].1 - coords[0].1) * beta
            };
            a.round();
            b.round();
            if a.x > b.x {
                swap(&mut a, &mut b);
                swap(&mut a_side_intensity, &mut b_side_intensity);
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
                    let p_int = a_side_intensity + (b_side_intensity - a_side_intensity) * phi;
                    let mut cur_color = color.clone();
                    cur_color.set_rgb(
                        (cur_color.r() as f64 * p_int) as u8,
                        (cur_color.g() as f64 * p_int) as u8,
                        (cur_color.b() as f64 * p_int) as u8,
                    );
                    self.set_pixel(p.x.round() as u32, p.y.round() as u32, cur_color);
                }
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let pixel = color.len() * (x + y * self.width) as usize;
        self.frame[pixel..pixel + color.len()].copy_from_slice(&color.to_array());
    }
}
