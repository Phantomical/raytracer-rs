
use vec::*;
use material::*;
use ray::*;

pub trait Intersectable {
	fn normal_at(&self, point : Vec3d) -> Vec3d;
	fn material_at(&self, point : Vec3d) -> &Material;
}

pub trait Raymarchable : Intersectable {
	fn distance(&self, point : Vec3d) -> f64;
}

pub trait Analytical : Intersectable { 
	fn nearest_intersect(&self, ray : Ray) -> Option<Vec3d>;
}
