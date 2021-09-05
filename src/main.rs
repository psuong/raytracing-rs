use std::{fs::File, io::Write};
use rand::prelude::*;
mod vec3;
mod ray;
mod math;
mod shapes;
mod hitable_list;
mod camera;

use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::ray::Ray;
use crate::shapes::{ HitRecord, Sphere };
use crate::vec3::Vec3;

/// Helper function to generate a random value between 0 and 1.
///
/// # Parameters
///
/// - `range` : i32
/// The maximum exclusive value that rand will generate from
///
/// # Returns
///
/// A normalized random value between [0..1)
fn generate_normalized_ran(range: i32) -> f32 {
    let random_value = rand::thread_rng().gen_range(0..range);
    return random_value as f32 / range as f32;
}

/// Picks a random point inside a unit sphere
///
/// # Returns
///
/// A random unit inside a unit sphere where all x, y, z
/// values are between [0..1).
fn random_in_unit_sphere() -> Vec3 {
    let mut p = 2.0 * Vec3::new(
        generate_normalized_ran(100), 
        generate_normalized_ran(100), 
        generate_normalized_ran(100)) - Vec3::from_uniform_value(1.0);

    while p.length_sq() >= 1.0 {
        p = 2.0 * Vec3::new(
            generate_normalized_ran(100), 
            generate_normalized_ran(100), 
            generate_normalized_ran(100)) - Vec3::from_uniform_value(1.0);
    }

    return p;
}

fn color<T: shapes::Hitable + Copy> (
    ray: &Ray, 
    world: &HitableList<T>) -> Vec3 {
    let mut rec = HitRecord::default();

    if hitable_list::hit(&world.list, &ray, 0.0, f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let new_ray = Ray::new(&rec.p, &(target - rec.p));

        return 0.5 * color(&new_ray, &world);
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
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)
    ];

    let world = HitableList::from_list(spheres);

    let mut output = File::create("image.ppm").expect("Unable to create file");
    let ppm_header = format!("P3\n{} {}\n255\n", nx, ny);

    output.write_all(ppm_header.as_bytes()).expect("Unable to write to file");

    let camera = Camera::new();

    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;
        while i < nx {
            let mut col = Vec3::from_uniform_value(0.0);

            let mut s = 0;
            while s < ns {
                let normalized_rand = generate_normalized_ran(100);

                let u = ((i as f32) + normalized_rand) / nx as f32;
                let v = ((j as f32) + normalized_rand) / ny as f32;

                let r = camera.get_ray(u, v);
                let _p = r.point_at_parameter(2.0); // Not really sure what I'm doing here
                col = col + color(&r, &world);
                s = s + 1;
            }

            col = col / ns;

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
