#![feature(clamp)]

mod camera;
mod hit;
mod ppm;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use hit::*;
use ppm::Ppm;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use std::{error::Error, fs::write};
use vec3::{Color, Point, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
const SAMPLES_PER_PIXEL: u16 = 100;
const MAX_DEPTH: u8 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    let camera = Camera::new();

    let mut world = HittableList::default();
    world.push(Rc::new(Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.push(Rc::new(Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let mut file = Ppm::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Height line: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += r.color(&world, MAX_DEPTH);
            }
            file.push_color(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    write("out.ppm", file)?;
    Ok(())
}
