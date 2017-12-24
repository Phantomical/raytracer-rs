
use vec::*;

const EPS : Vec2d = Vec2d{ x: 0.0, y: 0.000001 };

fn yxx(v : Vec2d) -> Vec3d {
	return Vec3d { x: v.y, y: v.x, z: v.x };
}
fn xyx(v : Vec2d) -> Vec3d {
	return Vec3d { x: v.x, y: v.y, z: v.x };
}
fn xxy(v : Vec2d) -> Vec3d {
	return Vec3d { x: v.x, y: v.x, z: v.y };
}

pub trait Raymarchable {
	fn normal_at(&self, point : Vec3d, _direction : Vec3d) -> Vec3d {
		// Normal approximation using gradient method

		return normalize(Vec3d{
			x: self.distance(point + yxx(EPS)) - self.distance(point - yxx(EPS)),
			y: self.distance(point + xyx(EPS)) - self.distance(point - xyx(EPS)),
			z: self.distance(point + xxy(EPS)) - self.distance(point - xxy(EPS))
		});

	}

	fn distance(&self, point : Vec3d) -> f64;
}
