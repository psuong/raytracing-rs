use std::{fmt::{self, Display, Result}, ops::{Add, Mul, Div, Sub }};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        return Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { x, y, z }
    }

    pub fn from_uniform_value(v: f32) -> Vec3 {
        return Vec3 { x: v, y: v, z: v }
    }

    #[allow(dead_code)]
    pub fn dot(self, rhs: Vec3) -> f32 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[allow(dead_code)]
    pub fn cross(self, other: Vec3) -> Vec3 {
        return Vec3 {
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

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        return *self / length;
    }

    pub fn length_sq(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f32 {
        return self.length_sq().sqrt()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        return Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        return Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        return Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        return Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self
        }
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: i32) -> Self::Output {
        return Vec3 {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
            z: self.z / rhs as f32
        }
    }
}
