use crate::color::Color;
pub struct Ppm(String);

impl Ppm {
    pub fn new(width: u16, height: u16) -> Self {
        Self(String::from(&format!("P3\n{} {}\n255\n", width, height)))
    }

    pub fn push_str(&mut self, s: &String) {
        self.0.push_str(s);
    }

    pub fn push_color(&mut self, c: &Color) {
        let c = *c * 255.999;
        let (r, g, b) = (c.r().trunc(), c.g().trunc(), c.b().trunc());
        let ic = Color::new(r, g, b);
        self.0.push_str(&format!("{}\n", ic));
    }
}

impl std::convert::AsRef<[u8]> for Ppm {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
