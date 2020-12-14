use crate::color::Color;
use crate::point::Point;
use crate::vec3::Vec3;

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
}
