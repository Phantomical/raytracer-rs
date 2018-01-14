
use lib::*;
use lib::object::*;

pub struct Hollow<T: Raymarchable> {
	obj : T
}

impl<T: Raymarchable> Hollow<T> {
	pub fn new(obj : T) -> Self {
		Self { obj }
	}
}

impl<T: Raymarchable> Raymarchable for Hollow<T> {
	fn distance(&self, point : Vec3d) -> f64 {
		self.obj.distance(point).abs()
	}

	fn normal_at(&self, point : Vec3d, dir : Vec3d) -> Vec3d {
		let dist = self.distance(point);

		self.obj.normal_at(point, dir) * if dist < 0.0 { -1.0 } else { 1.0 }
	}
}
