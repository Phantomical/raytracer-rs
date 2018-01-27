use vec::*;
use object::Raymarchable;

pub struct Mandelbulb {
    iterations: usize,
    power: i32,
}

impl Mandelbulb {
    pub fn new(iterations: usize, power: i32) -> Self {
        Self { iterations, power }
    }
}

impl Raymarchable for Mandelbulb {
    fn distance(&self, point: Vec3d) -> f64 {
        let mut w = point;
        let mut m = dot(w, w);
        let mut dz = 1.0;
        let power = self.power as f64;

        for _ in 0..self.iterations {
            dz = power * m.sqrt().powi(self.power - 1) * dz + 1.0;

            let r = length(w);
            let b = power * (w.y / r).acos();
            let a = power * w.x.atan2(w.z);

            w = point
                + r.powi(self.power) * Vec3d::new(b.sin() * a.sin(), b.cos(), b.sin() * a.cos());

            m = dot(w, w);
        }

        return 0.25 * m.ln() * m.sqrt() / dz;
    }
}