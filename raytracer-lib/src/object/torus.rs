
use lib::*;
use lib::object::*;

pub struct Torus {
	inner_radius : f64,
	outer_radius : f64
}

impl Torus {
	pub fn new(inner_radius : f64, outer_radius : f64) -> Torus {
		return Self { inner_radius, outer_radius };
	}
}

impl Raymarchable for Torus {
	fn distance(&self, point : Vec3d) -> f64 {
		let q = Vec2d::new(point.xz().magnitude() - self.inner_radius, point.y);
		return q.magnitude() - self.outer_radius;
	}
}
