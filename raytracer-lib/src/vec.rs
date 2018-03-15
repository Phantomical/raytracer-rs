use glslvec;

pub type Vec2u = glslvec::Vec2<u32>;
pub type Vec2d = glslvec::Vec2<f64>;
pub type Vec3d = glslvec::Vec3<f64>;
pub type Vec4d = glslvec::Vec4<f64>;
pub type Colour = glslvec::Vec3<f32>;
pub type Mat3d = glslvec::Mat3<f64>;

pub use glslvec::{abs, clamp, cross, distance, dot, length, max, min, mix, normalize, sqrt,
                  distance2, length2, vec2, vec3, vec4, reflect};

pub use glslvec::prelude::{HasLength, HasMinMax, HasNormalize};

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
