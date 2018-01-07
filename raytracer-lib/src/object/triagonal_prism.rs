
use lib::*;
use lib::object::*;

#[derive(Deserialize)]
pub struct TriangularPrism {
	height : f64,
	radius : f64
}

impl TriangularPrism {
	pub fn new(height : f64, radius : f64) -> Self {
		Self { height, radius }
	}
}

impl Raymarchable for TriangularPrism {
	fn distance(&self, p : Vec3d) -> f64 {
		let q = abs(p);
		return (q.z - self.height)
			.max((q.x*0.866025+p.y*0.5).max(-p.y) - self.radius * 0.5);
	}
}
