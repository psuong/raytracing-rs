use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin : Vec3,
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3
}

impl Camera {

    pub fn new() -> Camera {
        return Camera {
            origin : Vec3::from_uniform_value(0.0),
            lower_left_corner : Vec3::new(-2.0, -1.0, -1.0),
            vertical : Vec3::new(0.0, 2.0, 0.0),
            horizontal : Vec3::new(4.0, 0.0, 0.0)
        }
    }

    pub fn get_ray(self, u : f32, v : f32) -> Ray {
        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        return Ray::new(&self.origin, &dir);
    }
}
