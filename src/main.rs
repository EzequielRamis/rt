mod color;
mod hittable;
mod point;
mod ppm;
mod ray;
mod sphere;
mod vec3;
use color::Color;
use point::Point;
use ppm::Ppm;
use ray::Ray;
use std::{error::Error, fs::write};
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 400;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn main() -> Result<(), Box<dyn Error>> {
    let origin = Point::default();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let focal = Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal;

    let mut file = Ppm::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Height line: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            file.push_color(&ray_color(&ray));
        }
    }

    write("out.ppm", file)?;
    Ok(())
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r);
    if t.is_sign_positive() {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return (n + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = r.direction.unit_vector();
    let t = (unit_direction[1] + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    match discriminant.is_sign_positive() {
        true => (-half_b - discriminant.sqrt()) / a,
        false => -1.0,
    }
}
