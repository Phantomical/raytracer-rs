use lib::*;
use lib::object::*;

pub struct Cone {
    c: Vec2d,
}

impl Cone {
    pub fn new(radius: f64, height: f64) -> Self {
        Self {
            c: Vec2d::new(radius, height).normalize(),
        }
    }
}

impl Raymarchable for Cone {
    fn distance(&self, p: Vec3d) -> f64 {
        let q = p.xy().magnitude();
        return dot(self.c, Vec2d::new(q, p.z));
    }
}
