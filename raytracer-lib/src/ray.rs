
use vec::*;

pub struct Ray {
	pub origin    : Vec3d,
	pub direction : Vec3d,
}

impl Ray {
	fn new(orig : Vec3d, dir : Vec3d) -> Ray {
		return Ray {
			origin    : orig,
			direction : normalize(dir)
		};
	}

	fn point_at(&self, distance : f64) -> Vec3d {
		return self.origin + self.direction * distance;
	}
}
