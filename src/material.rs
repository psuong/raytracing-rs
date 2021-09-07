use crate::ray::Ray;
use crate::shapes::HitRecord;
use crate::vec3::Vec3;
use crate::random_utils::{random_in_unit_sphere, generate_normalized_ran};
use crate::math::{reflect, refract, dot, schlick};

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
    #[allow(unused_variables)]
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

#[derive(Clone, Copy)]
pub struct Dielectric {
    ref_idx : f32
}

#[allow(unused_assignments)]
#[allow(unused_assignments)]
impl Physics for Dielectric {
    fn scatter(
        self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool {

        let mut outward_normal = Vec3::zero();
        let mut ni_over_nt = 0.0;
        let reflected = reflect(ray.direction, rec.normal);
        
        *attenuation = Vec3::from_uniform_value(1.0);
        let mut refracted = Vec3::zero();

        let mut reflected_prob : f32 = 0.0;
        let mut cosine : f32 = 0.0;

        if dot(ray.direction, rec.normal) > 0.0 {
            outward_normal = -1.0 * rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(ray.direction, rec.normal) / ray.direction.length()
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -1.0 * dot(ray.direction, rec.normal) / ray.direction.length()
        }

        if refract(ray.direction, outward_normal, ni_over_nt, &mut refracted) {
            reflected_prob = schlick(cosine, self.ref_idx);
            *scattered = Ray::new(&rec.p, &refracted);
        } else {
            *scattered = Ray::new(&rec.p, &reflected);
            reflected_prob = 1.0;
        }

        if generate_normalized_ran(100) < reflected_prob {
            *scattered = Ray::new(&rec.p, &reflected);
        } else {
            *scattered = Ray::new(&rec.p, &refracted);
        }
        return true;
    }
}

impl Dielectric {
    pub fn new(reference_index: f32) -> Dielectric {
        return Dielectric {
            ref_idx: reference_index
        }
    }
}
