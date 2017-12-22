
extern crate cgmath;

pub type Vec3d = cgmath::Vector3<f64>;
pub type Colour = cgmath::Vector3<f32>;

pub use self::cgmath::dot;

pub fn norm(v : Vec3d) -> f64 {
	return cgmath::dot(v, v).sqrt();
}
pub fn norm2(v : Vec3d) -> f64 {
	return cgmath::dot(v, v);
}
pub fn normalize(v : Vec3d) -> Vec3d{ 
	return v * (1.0_f64 / norm(v));
}
