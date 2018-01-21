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

const BAILOUT: f64 = 1000.0;

impl Raymarchable for Sierpinski {
    fn distance(&self, point: Vec3d) -> f64 {
        let mut x = point.x;
        let mut y = point.y;
        let mut z = point.z;
        let mut r = x * x + y * y + z * z;

        for _ in 0..self.iterations {
            if x + y < 0.0 {
                let x1 = -y;
                y = -x;
                x = x1;
            }
            if x + z < 0.0 {
                let x1 = -z;
                z = -x;
                x = x1;
            }
            if y + z < 0.0 {
                let y1 = -z;
                z = -y;
                y = y1;
            }

            x = self.scale * x - (self.scale - 1.0);
            y = self.scale * y - (self.scale - 1.0);
            z = self.scale * z - (self.scale - 1.0);

            r = x * x + y * y + z * z;

            if r > BAILOUT {
                break;
            }
        }

        return (r.sqrt() - 2.0) / self.scale.powi(self.iterations as i32);
    }
}
