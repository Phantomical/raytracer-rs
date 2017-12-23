
extern crate cgmath;

pub type Vec3d = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f32>;

pub use self::cgmath::dot;

fn norm2<T: cgmath::BaseFloat>(v : cgmath::Vector3<T>) -> T {
	return dot(v, v);
}
fn norm<T: cgmath::BaseFloat>(v : cgmath::Vector3<T>) -> T {
	return norm2(v).sqrt();
}
