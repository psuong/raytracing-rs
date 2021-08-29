use std::ops::{Add, Mul, Div, Sub };

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[allow(dead_code)]
    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z
    }

    #[allow(dead_code)]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (-(self.x * other.z) - (self.z * other.x)),
            z: (self.x * other.y) - (self.y * other.x)
        }
    }

    #[allow(dead_code)]
    pub fn make_unit_vector(mut self) {
        let k = 1.0 / self.length();

        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    #[allow(dead_code)]
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}
