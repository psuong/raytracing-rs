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
        let dir = target - rec.p;

        // println!("Target: {}, Dir: {}", target, dir);
        *scattered = Ray::new(&rec.p, &dir);
        *attenuation = self.albedo.clone();
        return true;
    }
}


#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn with_properties(albedo_value: Vec3, f: f32) -> Metal {
        return Metal {
            albedo: albedo_value,
            fuzz: f
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
        
        let reflected = reflect(ray.direction, rec.normal);
        *scattered = Ray::new(&rec.p, &(reflected + self.fuzz * random_in_unit_sphere()));
        *attenuation = self.albedo.clone();
        return (dot(scattered.direction, rec.normal)) > 0.0;
    }
}
