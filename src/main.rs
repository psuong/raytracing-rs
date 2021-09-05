use std::{fs::File, io::Write};
mod vec3;
mod ray;
mod math;
mod shapes;
mod hitable_list;

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
    let nx : i32 = 200;
    let ny : i32 = 100;

    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal        = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical          = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin            = vec3::Vec3::single(0.0);

    let spheres = vec![
        shapes::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5),
        shapes::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0)
    ];

    let world = hitable_list::HitableList::from_list(spheres);

    let mut output = File::create("image.ppm").expect("Unable to create file");
    let ppm_header = format!("P3\n{} {}\n255\n", nx, ny);

    output.write_all(ppm_header.as_bytes()).expect("Unable to write to file");

    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;
        while i < nx {

            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            
            // Interpolate and casta ray to each pixel
            let r = ray::Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v));

            let _p = r.point_at_parameter(2.0);
            let color = color(&r, &world);

            // let color = color(&r);
            let ir : i32 = (255.99 * color.x) as i32;
            let ig : i32 = (255.99 * color.y) as i32;
            let ib : i32 = (255.99 * color.z) as i32;

            let color_row = format!("{} {} {}\n", ir, ig, ib);

            if ir < 0 || ig < 0 || ib < 0 {
                println!("{}", color_row);
            }

            output.write_all(color_row.as_bytes()).expect("Unable to write color");
            i = i + 1;
        }

        j = j - 1;
    }
}
