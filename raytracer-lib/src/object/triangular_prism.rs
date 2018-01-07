
use lib::*;
use lib::object::*;

pub struct TriangularPrism {
	h : Vec2d
}

impl TriangularPrism {
	pub fn new(height : f64, radius : f64) -> Self {
		Self { h: Vec2d::new(radius, height) }
	}
}

impl Raymarchable for TriangularPrism {
	fn distance(&self, p : Vec3d) -> f64 {
		let q = abs(p);
		return (q.z - self.h.y)
			.max((q.x*0.866025+p.y*0.5).max(-p.y) - h.x * 0.5);
	}
}
