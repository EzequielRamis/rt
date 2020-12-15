use crate::hit::*;
use crate::vec3::{Color, Point, Vec3};
use std::f64::INFINITY;

#[derive(Default, Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn color(&self, world: &HittableList, depth: u8) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {
            return Color::default();
        }

        if world.hit(self, 0.001, INFINITY, &mut rec) {
            let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
            return 0.5 * Ray::color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
