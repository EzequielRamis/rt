use vec3::Vec3;

use crate::vec3;

pub type Color = Vec3;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self([r, g, b])
    }

    pub fn r(&self) -> f64 {
        self[0]
    }

    pub fn g(&self) -> f64 {
        self[1]
    }

    pub fn b(&self) -> f64 {
        self[2]
    }
}
