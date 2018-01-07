
use lib::*;
use lib::object::*;

pub struct HexagonalPrism {
	h : Vec2d
}

impl HexagonalPrism {
	pub fn new(height : f64, radius : f64) -> Self {
		return Self { h: Vec2d::new(radius, height) };
	}
}

impl Raymarchable for HexagonalPrism {
	pub fn distance(&self, p : Vec3d) -> f64 {
		let q = abs(p);
		return (q.z-h.y).max((q.x*0.866025+q.y*0.5).max(q.y) - h.x)
	}
}
