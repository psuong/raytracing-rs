use std::{fs::File, io::Write};
use std::ops::{Add, Mul, Div, Sub };

struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z
    }

    fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (-(self.x * other.z) - (self.z * other.x)),
            z: (self.x * other.y) - (self.y * other.x)
        }
    }

    fn make_unit_vector(mut self) {
        let k = 1.0 / self.length();

        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f32 {
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

fn main() {
    let nx : i32 = 200;
    let ny : i32 = 100;

    let mut output = File::create("image.ppm").expect("Unable to create file");
    let ppm_header = format!("P3\n{} {}\n255\n", nx, ny);

    output.write_all(ppm_header.as_bytes()).expect("Unable to write to file");

    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;
        while i < nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);

            let ir : i32 = (255.99 * col.x) as i32;
            let ig : i32 = (255.99 * col.y) as i32;
            let ib : i32 = (255.99 * col.z) as i32;

            let color = format!("{} {} {}\n", ir, ig, ib);
            output.write_all(color.as_bytes()).expect("Unable to write color");
            i = i + 1;
        }

        j = j - 1;
    }
}

