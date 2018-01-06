
extern crate cgmath;

use self::cgmath::Vector3;
use self::cgmath::BaseFloat;

pub type Vec2u = cgmath::Vector2<u32>;
pub type Vec2d = cgmath::Vector2<f64>;
pub type Vec3d = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f32>;

pub use self::cgmath::dot;
pub use self::cgmath::prelude::*;

pub fn abs(v : Vec3d) -> Vec3d {
	return Vec3d::new(v.x.abs(), v.y.abs(), v.z.abs());
}
pub fn max(a : Vec3d, b : Vec3d) -> Vec3d {
	return Vec3d::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
}

pub fn norm2<T: BaseFloat>(v : Vector3<T>) -> T {
	return dot(v, v);
}
pub fn norm<T: BaseFloat>(v : Vector3<T>) -> T {
	return norm2(v).sqrt();
}

/// Calculates the projection of x onto y.
/// In mathematical notation this would be
/// written as proj_y (x).
/// 
/// The projection of x onto y is the component
/// of x that is parallel to y.
/// 
/// # Example
/// ```
/// let x = Vec3d::new(1.0, 1.0, 0.0);
/// let y = Vec3d::new(1.0, 0.0, 0.0);
/// 
/// assert_eq!(proj(x, y), y);
/// ```
///
pub fn proj(x : Vec3d, y : Vec3d) -> Vec3d {
	return (dot(x, y) / norm2(y)) * y;
}

pub fn perp(x : Vec3d, y : Vec3d) -> Vec3d {
	return x - proj(x, y);
}

/// Rotates a vector around an axis by and angle (in radians)
/// Source: https://math.stackexchange.com/questions/511370/how-to-rotate-one-vector-about-another
pub fn rotate(v : Vec3d, axis : Vec3d, angle : f64) -> Vec3d {
	let par = proj(v, axis);
	let perp = v - par;

	let w = Vec3d::cross(axis, perp).normalize();
	let pmag = norm(perp);

	let f = angle.sin_cos();	
	let rot = f.1 * perp + pmag * f.0 * w;

	return rot + par;
}

