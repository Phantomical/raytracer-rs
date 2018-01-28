use lib::*;
use lib::object::*;

pub struct Cylinder {
    c: Vec3d,
}

impl Cylinder {
    pub fn new(c: Vec3d) -> Self {
        Self { c }
    }
}

impl Raymarchable for Cylinder {
    fn distance(&self, p: Vec3d) -> f64 {
        return (p.xz() - self.c.xy()).length() - self.c.z;
    }
}
