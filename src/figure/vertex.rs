use std::ops::{Add, BitXor, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn world_to_screen(&self, height: u32, width: u32) -> Vertex {
        let mut res = Self {
            x: ((self.x + 1.) * (width as f64) / 4.),
            y: ((self.y + 1.) * (height as f64) / 4.),
            z: self.z,
        };
        res.round();

        res
    }

    pub fn round(&mut self) {
        self.x = self.x.round();
        self.y = self.y.round();
        self.z = self.z.round();
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
}

impl Add for Vertex {
    type Output = Vertex;

    fn add(self, other: Vertex) -> Vertex {
        Vertex {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vertex {
    type Output = Vertex;

    fn sub(self, other: Vertex) -> Vertex {
        Vertex {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl BitXor for Vertex {
    type Output = Vertex;

    fn bitxor(self, other: Vertex) -> Vertex {
        Vertex {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Mul for Vertex {
    type Output = f64;

    fn mul(self, other: Vertex) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vertex {
    type Output = Vertex;

    fn mul(self, other: f64) -> Vertex {
        Vertex {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
