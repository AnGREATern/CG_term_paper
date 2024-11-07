use crate::EPS;

#[derive(Clone)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn new(color: [u8; 4]) -> Self {
        Self(color[0], color[1], color[2], color[3])
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.0 = r;
        self.1 = g;
        self.2 = b;
    }

    pub fn a(&self) -> u8 {
        self.3
    }

    pub fn len(&self) -> usize {
        4
    }

    pub fn to_array(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn join_by_part(c1: Color, c2: Color, ratio: f64) -> Color {
        if ratio < EPS || (ratio - 1.) > EPS {
            panic!("Incorrect ratio");
        }
        Self(
            (c1.r() as f64 + (c2.r() as f64 - c1.r() as f64) * ratio).round() as u8,
            (c1.g() as f64 + (c2.g() as f64 - c1.g() as f64) * ratio).round() as u8,
            (c1.b() as f64 + (c2.b() as f64 - c1.b() as f64) * ratio).round() as u8,
            (c1.a() as f64 + (c2.a() as f64 - c1.a() as f64) * ratio).round() as u8,
        )
    }
}
