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

    pub fn perspective(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u).unit_vector();

        return Camera {
            origin : lookfrom,
            lower_left_corner : lookfrom - half_width * u - half_height * v - w,
            horizontal : 2.0 * half_width * u,
            vertical : 2.0 * half_height * v
        }
    }

    pub fn get_ray(self, u : f32, v : f32) -> Ray {
        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        return Ray::new(&self.origin, &dir);
    }
}
