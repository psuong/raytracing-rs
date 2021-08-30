use std::{fs::File, io::Write};
mod vec3;
mod ray;
mod math;

fn color(r: &ray::Ray) -> vec3::Vec3 {
    let center = vec3::Vec3::new(0.0, 0.0, -1.0);

    let t = hit_sphere(&center, 0.5, &r);
    if t > 0.0 {
        let n = ((*r).point_at_parameter(t) - center).unit_vector();
        return vec3::Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return vec3::Vec3::single(1.0) * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &vec3::Vec3, radius: f32, r: &ray::Ray) -> f32 {
    let oc = r.origin - *center;
    let a = math::dot(&r.direction, &r.direction);
    let b = math::dot(&oc, &r.direction) * 2.0;

    let r_square = radius * radius;
    let c = math::dot(&oc, &oc) - r_square;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn main() {
    let nx : i32 = 200;
    let ny : i32 = 100;

    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::Vec3::single(0.0);

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

            let color = color(&r);
            let ir : i32 = (255.99 * color.x) as i32;
            let ig : i32 = (255.99 * color.y) as i32;
            let ib : i32 = (255.99 * color.z) as i32;

            let color_row = format!("{} {} {}\n", ir, ig, ib);
            output.write_all(color_row.as_bytes()).expect("Unable to write color");
            i = i + 1;
        }

        j = j - 1;
    }
}
