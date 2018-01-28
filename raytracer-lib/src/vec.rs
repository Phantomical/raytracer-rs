use glslvec;

pub type Vec2u = glslvec::Vec2<u32>;
pub type Vec2d = glslvec::Vec2<f64>;
pub type Vec3d = glslvec::Vec3<f64>;
pub type Vec4d = glslvec::Vec4<f64>;
pub type Colour = glslvec::Vec3<f32>;
pub type Mat3d = cgmath::Matrix3<f64>;

use cgmath;
pub use cgmath::{Rotation, SquareMatrix};

pub use glslvec::{
	abs, max, min, length, cross, dot, normalize, length2, clamp,
	distance, distance2, vec2, vec3, vec4, sqrt, mix
};

pub use glslvec::prelude::{HasLength, HasNormalize, HasMinMax};

pub fn cgmath_vec(v: Vec3d) -> cgmath::Vector3<f64> {
	cgmath::Vector3::new(v.x, v.y, v.z)
}
pub fn glslvec_vec(v: cgmath::Vector3<f64>) -> Vec3d {
	vec3(v.x, v.y, v.z)
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
pub fn proj(x: Vec3d, y: Vec3d) -> Vec3d {
    return (dot(x, y) / length2(y)) * y;
}

pub fn perp(x: Vec3d, y: Vec3d) -> Vec3d {
    return x - proj(x, y);
}

pub fn rotate(v: Vec3d, axis: Vec3d, angle: f64) -> Vec3d {
    let par = proj(v, axis);
    let perp = v - par;

    let w = normalize(cross(axis, perp));
    let pmag = length(perp);

    let f = angle.sin_cos();
    let rot = f.1 * perp + pmag * f.0 * w;

    return rot + par;
}
