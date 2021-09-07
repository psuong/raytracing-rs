mod camera;
mod hitable_list;
mod material;
mod math;
mod random_utils;
mod ray;
mod shapes;
mod vec3;

use std::{fs::File, io::Write};
use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::material::{Lambertian, Metal, Physics, Dielectric};
use crate::ray::Ray;
use crate::shapes::{HitRecord, Sphere, MaterialAccessor};
use crate::vec3::Vec3;

fn color<T: shapes::Hitable + MaterialAccessor + Copy> (
    lambertians: &Vec<Lambertian>,
    metals: &Vec<Metal>,
    dielectics: &Vec<Dielectric>,
    ray: &Ray, 
    world: &HitableList<T>,
    depth: i32) -> Vec3 {

    let mut rec = HitRecord::default();

    if hitable_list::hit(&world.list, &ray, 0.001, f32::MAX, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::from_uniform_value(0.0);

        if rec.material_type == 0 {
            let lamb_mat : Lambertian = lambertians[rec.material_index as usize];
            if lamb_mat.scatter(&ray, &rec, &mut attenuation, &mut scattered) && depth < 50 {
                return attenuation * color(&lambertians, &metals, &dielectics, &scattered, &world, depth + 1);
            } else {
                return Vec3::zero();
            }
        } else if rec.material_type == 1 {
            let metal_mat : Metal = metals[rec.material_index as usize];
            if metal_mat.scatter(&ray, &rec, &mut attenuation, &mut scattered) && depth < 50 {
                return attenuation * color(&lambertians, &metals, &dielectics, &scattered, &world, depth + 1);
            } else {
                return Vec3::zero();
            }
        } else {
            let dialectic_metal : Dielectric = dielectics[rec.material_index as usize];
            if dialectic_metal.scatter(&ray, &rec, &mut attenuation, &mut scattered) && depth < 50 {
                return attenuation * color(&lambertians, &metals, &dielectics, &scattered, &world, depth + 1);
            } else {
                return Vec3::zero();
            }
        }

        // Material materials
        // let target = rec.p + rec.normal + random_utils::random_in_unit_sphere();
        // let new_ray = Ray::new(&rec.p, &(target - rec.p));
        // return 0.5 * color(lambertians, metals, &new_ray, &world);
    } else {
        let unit_dir = ray.direction.unit_vector();
        let t = (unit_dir.y + 1.0) * 0.5;
        return (1.0 - t) * Vec3::from_uniform_value(1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx : i32 = 400; // width
    let ny : i32 = 200; // height
    let ns : i32 = 100; // AA sampling

    let spheres = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5).with_material(0, 0),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).with_material(0, 1),
        Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5).with_material(1, 0),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5).with_material(2, 0),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45).with_material(2, 0)
    ];

    let lambertians = vec![
        Lambertian::with_albedo(Vec3::new(0.1, 0.2, 0.5)),
        Lambertian::with_albedo(Vec3::new(0.8, 0.8, 0.0))
    ];

    let metals = vec![
        Metal::with_properties(Vec3::new(0.8, 0.6, 0.2), 0.3),
    ];

    let dielectric = vec![
        Dielectric::new(1.5)
    ];

    let world = HitableList::from_list(spheres);

    let mut output = File::create("image.ppm").expect("Unable to create file");
    let ppm_header = format!("P3\n{} {}\n255\n", nx, ny);

    output.write_all(ppm_header.as_bytes()).expect("Unable to write to file");

    let camera = Camera::perspective(
        Vec3::new(-2.0, 2.0, 1.0), 
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0), 
        90.0, 
        nx as f32 / ny as f32);

    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;
        while i < nx {
            let mut col = Vec3::from_uniform_value(0.0);

            let mut s = 0;
            while s < ns {
                let normalized_rand = random_utils::generate_normalized_ran(100);

                let u = ((i as f32) + normalized_rand) / nx as f32;
                let v = ((j as f32) + normalized_rand) / ny as f32;

                let r = camera.get_ray(u, v);
                let _p = r.point_at_parameter(2.0); // Not really sure what I'm doing here
                col = col + color(&lambertians, &metals, &dielectric, &r, &world, 0);
                s = s + 1;
            }

            col = col / ns;
            col = math::sqrt(col);

            let ir : i32 = (255.99 * col.x) as i32;
            let ig : i32 = (255.99 * col.y) as i32;
            let ib : i32 = (255.99 * col.z) as i32;

            let color_row = format!("{} {} {}\n", ir, ig, ib);
            output.write_all(color_row.as_bytes()).expect("Unable to write color");
            i = i + 1;
        }

        j = j - 1;
    }
}
