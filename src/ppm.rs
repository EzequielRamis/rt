use crate::vec3::Color;
pub struct Ppm(String);

impl Ppm {
    pub fn new(width: u16, height: u16) -> Self {
        Self(String::from(&format!("P3\n{} {}\n255\n", width, height)))
    }

    pub fn push_str(&mut self, s: &String) {
        self.0.push_str(s);
    }

    pub fn push_color(&mut self, c: &Color, samples_per_pixel: u16) {
        let scale = (samples_per_pixel as f64).recip();
        let (r, g, b) = {
            let c = *c * scale;
            let clamp = |x: f64| -> f64 { (256.0 * x.sqrt().clamp(0.0, 0.999)).trunc() };
            (clamp(c.x()), clamp(c.y()), clamp(c.z()))
        };
        let ic = Color::new(r, g, b);
        self.0.push_str(&format!("{}\n", ic));
    }
}

impl std::convert::AsRef<[u8]> for Ppm {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
