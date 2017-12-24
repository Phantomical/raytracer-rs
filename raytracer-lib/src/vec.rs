
extern crate cgmath;

pub type Vec2d = cgmath::Vector2<f64>;
pub type Vec3d = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f32>;

pub use self::cgmath::dot;

pub fn norm2<T: cgmath::BaseFloat>(v : cgmath::Vector3<T>) -> T {
	return dot(v, v);
}
pub fn norm<T: cgmath::BaseFloat>(v : cgmath::Vector3<T>) -> T {
	return norm2(v).sqrt();
}
pub fn normalize<T: cgmath::BaseFloat>(v : cgmath::Vector3<T>) -> cgmath::Vector3<T> {
	return v * norm(v).recip();
}
