use image::{RgbImage, Rgb, ImageBuffer};
use std::time::Instant;
use num::Complex;
use rayon::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MAX_ITERATIONS: u32 = 100;
const CONFIG: Complex<f64> = Complex::new(-0.4, -0.59);

fn range_map(value: u32, dimension: u32, out_min: f64, out_max: f64) -> f64 {
    (value as f64 / dimension as f64) * (out_max - out_min) + out_min
}

fn compute_membership(x: f64, y: f64) -> u32 {
    let mut z = Complex::new(x, y);
    let mut iteration_count = 0;

    while z.norm() < 2.0 && iteration_count < MAX_ITERATIONS {
        z = z * z + CONFIG;
        iteration_count += 1;
    }

    let proportion = iteration_count as f64 / MAX_ITERATIONS as f64;
    ((1.0 - proportion) * 255.0) as u32
}

fn main() {
    let mut julia: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let start = Instant::now();
    julia.enumerate_pixels_mut().par_bridge().for_each(|(c, r, pixel)| {
        let x = range_map(c, WIDTH, -1.0, 1.0);
        let y = range_map(r, HEIGHT, -1.0, 1.0);
        let membership = compute_membership(x, y);

        *pixel = u32_to_rgb(membership);
    });

    let elapsed = start.elapsed();
    println!("elapsed: {:?}", elapsed);

    julia.save("julia.png").unwrap();
}

fn u32_to_rgb(color: u32) -> Rgb<u8> {
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;

    Rgb([red, green, blue])
}
