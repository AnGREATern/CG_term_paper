use crate::DEFAULT_SCALE;

use std::f64::{MAX, MIN};
use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, Neg, Sub, SubAssign};
use std::cmp::Ordering;

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
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
            x: ((self.x + 1.) * (width as f64) / DEFAULT_SCALE),
            y: ((self.y + 1.) * (height as f64) / DEFAULT_SCALE),
            z: self.z,
        };
        res.round();

        res
    }

    pub fn project_to_sphere(self, center: Vertex, radius: f64) -> Vertex {
        let dir = self - center;

        dir * (radius / dir.len())
    }

    pub fn round(&mut self) {
        self.x = self.x.round();
        self.y = self.y.round();
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn len2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn triple_prod(a: Vertex, b: Vertex, c: Vertex) -> f64 {
        (b ^ c) * a
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn mov(&mut self, delta: Vertex) {
        *self = *self + delta;
    }

    pub fn rotate(&mut self, angles: Vertex) {
        let (sin_x, cos_x) = angles.x.sin_cos();
        let (sin_y, cos_y) = angles.y.sin_cos();
        let (sin_z, cos_z) = angles.z.sin_cos();

        let y1 = self.y * cos_x - self.z * sin_x;
        let z1 = self.y * sin_x + self.z * cos_x;

        let x2 = self.x * cos_y + z1 * sin_y;
        let z2 = -self.x * sin_y + z1 * cos_y;

        let x3 = x2 * cos_z - y1 * sin_z;
        let y3 = x2 * sin_z + y1 * cos_z;

        self.x = x3;
        self.y = y3;
        self.z = z2;
    }

    pub fn bounding_box(verts: &Vec<Self>) -> (Self, Self) {
        let mut bbox = (Self::new(MAX, MAX, MAX), Self::new(MIN, MIN, MIN));
        for v in verts {
            if v.x < bbox.0.x {
                bbox.0.x = v.x
            }
            if v.y < bbox.0.y {
                bbox.0.y = v.y
            }
            if v.z < bbox.0.z {
                bbox.0.z = v.z
            }

            if v.x > bbox.1.x {
                bbox.1.x = v.x
            }
            if v.y > bbox.1.y {
                bbox.1.y = v.y
            }
            if v.z > bbox.1.z {
                bbox.1.z = v.z
            }
        }

        bbox
    }

    pub fn max(self) -> f64 {
        if self.x > self.y && self.x > self.z {
            self.x
        } else if self.y > self.z {
            self.y
        } else {
            self.z
        }
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

impl AddAssign for Vertex {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vertex {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl DivAssign<usize> for Vertex {
    fn div_assign(&mut self, other: usize) {
        self.x /= other as f64;
        self.y /= other as f64;
        self.z /= other as f64;
    }
}

impl DivAssign<f64> for Vertex {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
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

impl Neg for Vertex {
    type Output = Vertex;

    fn neg(self) -> Vertex {
        Vertex {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

impl Div<f64> for Vertex {
    type Output = Vertex;

    fn div(self, other: f64) -> Vertex {
        Vertex {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y, self.z)
            .partial_cmp(&(other.x, other.y, other.z))
            .unwrap_or(Ordering::Equal)
    }
}
