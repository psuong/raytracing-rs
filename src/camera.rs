use crate::random_utils::random_in_unit_disk;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin : Vec3,
    pub lower_left_corner : Vec3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub u : Vec3,
    pub v : Vec3,
    pub w: Vec3,
    lens_radius : f32,
}

impl Camera {

    pub fn perspective(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u).unit_vector();

        return Camera {
            origin : lookfrom,
            lower_left_corner : lookfrom - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal : 2.0 * half_width * u * focus_dist,
            vertical : 2.0 * half_height * v * focus_dist,
            u : u,
            v : v,
            w : w,
            lens_radius : aperture / 2.0
        }
    }

    pub fn get_ray(self, s : f32, t : f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(
            &(self.origin + offset), 
            &(self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset));
    }
}
