
use lib::*;
use lib::object::Raymarchable;

pub struct Transform<T: Raymarchable> {
	obj : T,
	mat : Mat3d
}

impl<T: Raymarchable> Transform<T> {
	pub fn new(mat : Mat3d, obj : T) -> Self {
		Self { obj, mat }
	}
}

impl<T: Raymarchable> Raymarchable for Transform<T> {
	fn distance(&self, point : Vec3d) -> f64 {
		self.obj.distance(self.mat * point)
	}
	fn normal_at(&self, point : Vec3d, dir : Vec3d) -> Vec3d {
		self.obj.normal_at(self.mat * point, (self.mat * dir).normalize())
	}
}
