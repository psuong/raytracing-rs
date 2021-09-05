use std::{fs::File, io::Write};
use rand::prelude::*;
mod vec3;
mod ray;
mod math;
mod shapes;
mod hitable_list;
mod camera;

fn color<T: shapes::Hitable + Copy> (
    ray: &ray::Ray, 
    world: &hitable_list::HitableList<T>) -> vec3::Vec3 {
    let mut rec = shapes::HitRecord::default();

    if hitable_list::hit(&world.list, &ray, 0.0, f32::MAX, &mut rec) {
        let ret_val = 0.5 * (vec3::Vec3::new(
            rec.normal.x, 
            rec.normal.y, 
            rec.normal.z) + vec3::Vec3::single(1.0));

        if ret_val.x < 0.0 {
            println!("Normal -> {}", ret_val.x);
        }
        return ret_val;
    } else {
        let unit_dir = ray.direction.unit_vector();
        let t = (unit_dir.y + 1.0) * 0.5;
        let ret_val = (1.0 - t) * vec3::Vec3::single(1.0) + t * vec3::Vec3::new(0.5, 0.7, 1.0);

        if ret_val.x < 0.0 {
            println!("Else -> {}", ret_val.x);
        }

        return ret_val;
    }
}

fn main() {
    let nx : i32 = 400; // width
    let ny : i32 = 200; // height
    let ns : i32 = 100; // AA sampling

    let spheres = vec![
        shapes::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5),
        shapes::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0)
    ];

    let world = hitable_list::HitableList::from_list(spheres);

    let mut output = File::create("image.ppm").expect("Unable to create file");
    let ppm_header = format!("P3\n{} {}\n255\n", nx, ny);

    output.write_all(ppm_header.as_bytes()).expect("Unable to write to file");

    let camera = camera::Camera::new();

    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;
        while i < nx {
            let mut col = vec3::Vec3::single(0.0);

            let mut s = 0;
            while s < ns {
                let random_value : i32 = rand::thread_rng().gen_range(0..100);
                let normalized_rand = (random_value as f32) / 100.0;

                let u = ((i as f32) + normalized_rand) / nx as f32;
                let v = ((j as f32) + normalized_rand) / ny as f32;

                let r = camera.get_ray(u, v);
                let _p = r.point_at_parameter(2.0); // Not really sure what I'm doing here
                col = col + color(&r, &world);
                s = s + 1;
            }

            col = col / ns;

            // Interpolate and cast a ray to each pixel
            // let r = ray::Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v));
            
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
