use std::{fs::File, io::{Write}};

fn main() {
    let nx : i32 = 200;
    let ny : i32 = 100;

    let mut output = File::create("image.ppm").expect("Unable to create file");
    let ppm_header = format!("P3\n{} {}\n255\n", nx, ny);

    output.write_all(ppm_header.as_bytes()).expect("Unable to write to file");

    let mut j = ny - 1;
    while j >= 0 {
        let mut i = 0;
        while i < nx {
            let r : f32 = i as f32 / nx as f32;
            let g : f32 = j as f32 / ny as f32;
            let b : f32 = 0.2;

            let ir : i32 = (255.99 * r) as i32;
            let ig : i32 = (255.99 * g) as i32;
            let ib : i32 = (255.99 * b) as i32;

            let color = format!("{} {} {}\n", ir, ig, ib);
            output.write_all(color.as_bytes()).expect("Unable to write color");
            i = i + 1;
        }

        j = j - 1;
    }
}

