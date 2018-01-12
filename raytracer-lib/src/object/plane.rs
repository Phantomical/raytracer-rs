
use vec::*;
use object::*;

pub struct Plane {
	/// The normal vector of the plane.
	/// All vectors laying on the plane
	/// are perpendicular to this vector.
	normal : Vec3d,
	// A point on the plane
	point  : Vec3d
}

impl Plane {
	pub fn new(normal : Vec3d, point : Vec3d) -> Plane {
		return Plane {
			normal: normal.normalize(),
			point:  point 
		};
	}
}

impl Analytical for Plane {}     

impl Raymarchable for Plane {
	fn distance(&self, point : Vec3d) -> f64 {
		return dot(self.normal, point - self.point);
	}

	fn normal_at(&self, _point : Vec3d, _dir : Vec3d) -> Vec3d {
		return self.normal;
	}
}
