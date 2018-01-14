use lib::*;
use lib::object::*;

pub struct Repeat<T: Raymarchable> {
    modulus: Vec3d,
    obj: T,
}

impl<T: Raymarchable> Repeat<T> {
    pub fn new(obj: T, modulus: Vec3d) -> Self {
        Self { obj, modulus }
    }
}

fn modulus(v: Vec3d, m: Vec3d) -> Vec3d {
    Vec3d::new(
        if m.x.is_zero() { v.x } else { v.x % m.x },
        if m.y.is_zero() { v.y } else { v.y % m.y },
        if m.z.is_zero() { v.z } else { v.z % m.z },
    )
}

impl<T: Raymarchable> Raymarchable for Repeat<T> {
    fn distance(&self, point: Vec3d) -> f64 {
        self.obj
            .distance(modulus(point, self.modulus) - (self.modulus * 0.5))
    }

    fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
        self.obj
            .normal_at(modulus(point, self.modulus) - (self.modulus * 0.5), dir)
    }
}
