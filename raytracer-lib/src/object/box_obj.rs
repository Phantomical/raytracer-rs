
use lib::*;
use lib::object::*;

pub struct Box {
	bounds   : Vec3d
}

impl Raymarchable for Box {
	fn distance(&self, point : Vec3d) -> f64 {
		let d = abs(point) - self.bounds;

		return d.x.max(d.y.max(d.z)).min(0.0)
			+ norm(max(d, Vec3d::zero()));
	}
}
