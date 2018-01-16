use lib::*;
use lib::object::Raymarchable;

pub struct Sierpinski {
    iterations: u32,
    scale: f64,
}

impl Sierpinski {
    pub fn new(iterations: u32, scale: f64) -> Self {
        Self { iterations, scale }
    }
}

const A1: Vec3d = Vec3d {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const A2: Vec3d = Vec3d {
    x: -1.0,
    y: -1.0,
    z: 1.0,
};
const A3: Vec3d = Vec3d {
    x: 1.0,
    y: -1.0,
    z: -1.0,
};
const A4: Vec3d = Vec3d {
    x: -1.0,
    y: 1.0,
    z: -1.0,
};

impl Raymarchable for Sierpinski {
    fn distance(&self, point: Vec3d) -> f64 {
        let mut z = point;

        for _ in 0..self.iterations {
            let mut dist = (z - A1).magnitude();
            let mut d: f64;
            let mut c = A1;

            d = (z - A2).magnitude();
            if d < dist {
                c = A2;
                dist = d;
            }

            d = (z - A3).magnitude();
            if d < dist {
                c = A3;
                dist = d;
            }

            d = (z - A4).magnitude();
            if d < dist {
                c = A4
            }

            z = z * self.scale - c * (self.scale - 1.0);
        }

        return z.magnitude() / self.scale.powi(self.iterations as i32);
    }
}
