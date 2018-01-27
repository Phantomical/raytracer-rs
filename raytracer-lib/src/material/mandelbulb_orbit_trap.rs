use lib::*;
use lib::material::Material;

//use gradient::Gradient;

pub struct MandelbulbOrbitTrap {
    //gradient : Gradient<f32, Colour>,
    power: i32,
    iterations: usize,
}

impl MandelbulbOrbitTrap {
    pub fn new(
        //gradient : Gradient<f32, Colour>,
        iterations: usize,
        power: i32,
    ) -> Self {
        Self {
            //gradient,
            power,
            iterations,
        }
    }
}

fn mk_vec4(xyz: Vec3d, w: f64) -> Vec4d {
    Vec4d::new(xyz.x, xyz.y, xyz.z, w)
}
pub fn min(a: Vec4d, b: Vec4d) -> Vec4d {
    Vec4d::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z), a.w.min(b.w))
}

fn mix(a: Vec3d, b: Vec3d, f: f64) -> Vec3d {
    a * f + b * (1.0 - f)
}
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    v.max(lo).min(hi)
}
fn sqrt(v: Vec3d) -> Vec3d {
    Vec3d::new(v.x.sqrt(), v.y.sqrt(), v.z.sqrt())
}

impl Material for MandelbulbOrbitTrap {
    fn base_colour(&self, isect: &Intersection) -> Colour {
        let mut w = isect.point;
        let mut m = dot(w, w);
        let power = self.power as f64;
        let mut trap = mk_vec4(abs(w), m);

        for _ in 0..self.iterations {
            let r = length(w);
            let b = power * (w.y / r).acos();
            let a = power * w.x.atan2(w.z);

            w = isect.point
                + r.powi(self.power) * Vec3d::new(b.sin() * a.sin(), b.cos(), b.sin() * a.cos());

            trap = min(trap, mk_vec4(abs(w), m));

            m = dot(w, w);
        }

        trap = Vec4d::new(m, trap.y, trap.z, trap.w).xwyz();

        let mut col = Vec3d::new(0.01, 0.01, 0.01);
        col = mix(col, Vec3d::new(0.10, 0.20, 0.30), clamp(trap.y, 0.0, 1.0));
        col = mix(
            col,
            Vec3d::new(0.02, 0.10, 0.30),
            clamp(trap.z * trap.z, 0.0, 1.0),
        );
        col = mix(
            col,
            Vec3d::new(0.30, 0.10, 0.02),
            clamp(trap.w.powi(6), 0.0, 1.0),
        );
        col *= 0.5;
        col = sqrt(col);

        return Colour::new(col.x as f32, col.y as f32, col.z as f32);
    }
}
