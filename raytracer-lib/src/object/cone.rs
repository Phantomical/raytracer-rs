use lib::*;
use lib::object::*;

#[derive(Copy, Clone)]
pub struct Cone {
    c: Vec2d,
}

impl Cone {
    pub fn new(radius: f64, height: f64) -> Self {
        Self {
            c: vec2(radius, height).normalize(),
        }
    }
}

impl Raymarchable for Cone {
    fn distance(&self, p: Vec3d) -> f64 {
        let q = p.xy().length();
        return dot(self.c, vec2(q, p.z));
    }
}
