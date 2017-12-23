
use vec::*;
use ray::*;
use object::*;

pub struct Sphere {
	pub centre : Vec3d,
	pub radius : f64
}

fn sqr(x : f64) -> f64 {
	return x * x;
}

impl Intersectable for Sphere {
	fn normal_at(&self, point : Vec3d) -> Vec3d {
		return normalize(point - self.centre);
	}
}

impl Analytical for Sphere {
	// Formula from here: 
	// https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
	fn nearest_intersect(&self, ray : Ray) -> Option<Vec3d> {
				let dot1 = dot(ray.direction, ray.origin - self.centre);
		let det = sqr(dot1) + sqr(self.radius) - norm2(ray.origin - self.centre);

		if det < 0.0 {
			return None;
		}

		let sqrtdet = det.sqrt();

		let d1 = -dot1 + sqrtdet;
		let d2 = -dot1 - sqrtdet;

		let d : f64;

		if d2 < 0.0 {
			if d1 < 0.0 {
				return None;
			}
			d = d1;
		}
		else {
			d = d2;
		}

		return Some(ray.point_at(d));
	}
}
