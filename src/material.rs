use crate::ray::Ray;
use crate::shapes::HitRecord;
use crate::vec3::Vec3;
use crate::random_utils::random_in_unit_sphere;
use crate::math::{ reflect, dot };

pub trait Physics {
    fn scatter(
        self, 
        ray: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Vec3, 
        scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn with_albedo(albedo_value: Vec3) -> Lambertian {
        return Lambertian {
            albedo: albedo_value
        }
    }
}

impl Physics for Lambertian {
    fn scatter(
        self, 
        ray: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Vec3, 
        scattered: &mut Ray) -> bool {

        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(&rec.p, &(target - rec.p));
        *attenuation = self.albedo;
        return true;
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn with_albedo(albedo_value: Vec3) -> Metal {
        return Metal {
            albedo: albedo_value
        }
    }
}

impl Physics for Metal {
    fn scatter(
        self, 
        ray: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Vec3, 
        scattered: &mut Ray) -> bool {
        
        let lhs = ray.direction.unit_vector();
        let reflected = reflect(lhs, rec.normal);
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        return dot(scattered.direction, rec.normal) > 0.0;
    }
}
