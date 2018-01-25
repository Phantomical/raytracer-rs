
use vec::*;
use cgmath::Basis3;

use object::Raymarchable;

pub struct Rotate<T: Raymarchable> {
	mat : Basis3<f64>,
	//inv : Basis3<f64>,
	obj : T,
}

impl<T: Raymarchable> Rotate<T> {
	pub fn new(obj : T, mat : Basis3<f64>) -> Self {
		Self {
			obj,
			mat,
			//inv: mat.inverse().expect("Rotation matrix was not invertible")
		}
	}
}

impl<T: Raymarchable> Raymarchable for Rotate<T> {
	fn distance(&self, point : Vec3d) -> f64 {
		self.obj.distance(self.mat.rotate_vector(point))
	}
}
