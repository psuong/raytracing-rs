use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::math;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32
}

pub trait Hitable {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::single(0.0),
            normal: Vec3::single(0.0)
        }
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;

        let a = math::dot(&r.direction, &r.direction);
        let b = math::dot(&oc, &r.direction) * 2.0;
    
        let r_square = self.radius * self.radius;
        let c = math::dot(&oc, &oc) - r_square;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if temp < t_max && temp > t_min {
                hit_record.p = r.point_at_parameter(temp);
                hit_record.t = temp;
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return true;
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                hit_record.p = r.point_at_parameter(temp);
                hit_record.t = temp;
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return true;
            }
        }

        return false;
    }
}
