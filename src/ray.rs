
use vec::*;

pub struct Ray {
	origin    : Vec3d,
	direction : Vec3d,
}

impl Ray {
	fn new(origin : Vec3d, direction : Vec3d) -> Ray {
		return Ray {
			origin:    origin,
			direction: direction
		};
	}
}
