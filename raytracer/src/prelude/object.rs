
use ndarray::*;
use array_vec::ArrayVec3;
use vec::{vec3, Vec3d};

/// Calculates the 
pub fn finite_difference_normal<T>(
	obj: &T, 
	points: &ArrayVec3) -> ArrayVec3
	where T: Object + ?Sized,
{
	let eps = obj.epsilon();
	let yxx = ArrayVec3::from(vec3(eps, 0.0, 0.0));
	let xyx = ArrayVec3::from(vec3(0.0, eps, 0.0));
	let xxy = ArrayVec3::from(vec3(0.0, 0.0, eps));

	ArrayVec3::new(
		obj.distance(&(points + &yxx)) - obj.distance(&(points - &yxx)),
		obj.distance(&(points + &xyx)) - obj.distance(&(points - &xyx)),
		obj.distance(&(points + &xxy)) - obj.distance(&(points - &xxy))
	)
}

pub trait Object {
	fn epsilon(&self) -> f64 {
		1.0e-6
	}

	fn distance(&self, points: &ArrayVec3) -> Array<f64, IxDyn>;

	fn normal_at(&self, points: &ArrayVec3) -> ArrayVec3
	{
		finite_difference_normal(self, points)
	}
}

impl Object {
	pub fn distance_singular(&self, point: Vec3d) -> f64 {
		self.distance(&ArrayVec3::from(point))[*&[0]]
	}
	pub fn normal_singular(&self, point: Vec3d) -> Vec3d {
		self.normal_at(&ArrayVec3::from(point)).at(*&[0])
	}
}