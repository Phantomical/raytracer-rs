
use vec::*;
use ray::*;

pub trait Intersectable {
	fn normal_at(&self, point : Vec3d) -> Vec3d;
}

pub trait Raymarchable: Intersectable {
	fn distance(&self, point : Vec3d) -> f64;
}

pub trait Analytical: Intersectable {
	fn nearest_intersect(&self, ray : Ray) -> Option<Vec3d>;
}
