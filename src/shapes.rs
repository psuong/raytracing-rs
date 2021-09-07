use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::math;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material_index: i32,
    pub material_type: i32      // TODO: Use an enum
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material_type: i32,
    material_index: i32
}

pub trait Hitable {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

pub trait MaterialAccessor {
    fn get_material_info(self) -> (i32, i32);
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::from_uniform_value(0.0),
            normal: Vec3::from_uniform_value(0.0),
            material_index: -1,
            material_type: -1
        }
    }
}

impl std::fmt::Display for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(t: {}, Point: {}, {}, {}, Normal: {}, {}, {})", 
            self.t, 
            self.p.x, self.p.y, self.p.z, 
            self.normal.x, self.normal.y, self.normal.z)
    }
}

impl Sphere {
    pub fn new(cent: Vec3, r: f32) -> Sphere {
        return Sphere { 
            center: cent, 
            radius: r, 
            material_type: -1, 
            material_index: -1 
        }
    }

    pub fn with_material(mut self, mat_type: i32, index: i32) -> Sphere {
        self.material_type = mat_type;
        self.material_index = index;
        return self;
    }
}

impl Hitable for Sphere {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;

        let a = math::dot(r.direction, r.direction);
        let b = math::dot(oc, r.direction);
        let c = math::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.p = r.point_at_parameter(temp);
                rec.t = temp;
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                rec.p = r.point_at_parameter(temp);
                rec.t = temp;
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }

        return false;
    }
}

impl MaterialAccessor for Sphere {
    fn get_material_info(self) -> (i32, i32) {
        return (self.material_type, self.material_index);
    }
}
