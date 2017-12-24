
use vec::*;
use ray::*;
use object::*;

pub struct Sphere {
	pub centre : Vec3d,
	pub radius : f64
}

impl Sphere {
	pub fn new(centre : Vec3d, radius : f64) -> Sphere {
		return Sphere {
			centre: centre,
			radius: radius
		}
	}
}

impl Raymarchable for Sphere {
	fn normal_at(&self, point : Vec3d, _dir : Vec3d) -> Vec3d {
		return normalize(point - self.centre);
	}
	fn distance(&self, point : Vec3d) -> f64 {
		return norm(point - self.centre) - self.radius;
	}
}
