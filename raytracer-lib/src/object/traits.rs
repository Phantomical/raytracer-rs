
use vec::*;
use ray::*;

fn yxx(v : Vec2d) -> Vec3d {
	return v.yxx();
}
fn xyx(v : Vec2d) -> Vec3d {
	return v.xyx();
}
fn xxy(v : Vec2d) -> Vec3d {
	return v.xxy();
}


pub trait Raymarchable: Sync + Send {
	fn epsilon(&self, _point : Vec3d) -> f64 {
		0.000001
	}

	fn normal_at(&self, point : Vec3d, _direction : Vec3d) -> Vec3d {
		let eps = Vec2d::new(0.0, self.epsilon(point));
		// Normal approximation using gradient method

		return Vec3d{
			x: self.distance(point + yxx(eps)) - self.distance(point - yxx(eps)),
			y: self.distance(point + xyx(eps)) - self.distance(point - xyx(eps)),
			z: self.distance(point + xxy(eps)) - self.distance(point - xxy(eps))
		}.normalize();
	}

	fn distance(&self, point : Vec3d) -> f64;

}

pub trait Analytical: Raymarchable {
	// Indicates that the object can find intersections analytically
	fn analytical(&self) -> bool {
		return false;
	}
	// Finds the intersection analytically
	fn intersect(&self, _ray : &Ray) -> Option<Vec3d> {
		unimplemented!();
	}
}
