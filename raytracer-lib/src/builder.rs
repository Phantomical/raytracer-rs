use lib::{Colour, Mat3d, Vec2d, Vec3d};
use lib::light::*;
use lib::object::*;
use lib::material::*;

use std::sync::Arc;
use std::ops::Deref;
use std::f64::consts::PI;

impl Raymarchable for Arc<Raymarchable> {
    fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
        return self.deref().normal_at(point, dir);
    }
    fn distance(&self, point: Vec3d) -> f64 {
        return self.deref().distance(point);
    }
}

pub fn vec2d(vals: [f64; 2]) -> Vec2d {
    return Vec2d::new(vals[0], vals[1]);
}
pub fn vec3d(vals: [f64; 3]) -> Vec3d {
    return Vec3d::new(vals[0], vals[1], vals[2]);
}
pub fn colour(vals: [f32; 3]) -> Colour {
    return Colour::new(vals[0], vals[1], vals[2]);
}

/* Objects */
pub fn sphere(pos: [f64; 3], radius: f64) -> Arc<Raymarchable> {
    return Arc::new(Sphere::new(vec3d(pos), radius));
}
pub fn box_obj(bounds: [f64; 3]) -> Arc<Raymarchable> {
    return Arc::new(BoxObj::new(vec3d(bounds)));
}
pub fn cone(radius: f64, height: f64) -> Arc<Raymarchable> {
    Arc::new(Cone::new(radius, height))
}
pub fn cylinder(c: [f64; 3]) -> Arc<Raymarchable> {
    Arc::new(Cylinder::new(vec3d(c)))
}
pub fn hexagonal_prism(height: f64, radius: f64) -> Arc<Raymarchable> {
    Arc::new(HexagonalPrism::new(height, radius))
}
pub fn plane(normal: [f64; 3], point: [f64; 3]) -> Arc<Raymarchable> {
    Arc::new(Plane::new(vec3d(normal), vec3d(point)))
}
pub fn torus(inner: f64, outer: f64) -> Arc<Raymarchable> {
    Arc::new(Torus::new(inner, outer))
}
pub fn triangular_prism(height: f64, radius: f64) -> Arc<Raymarchable> {
    Arc::new(TriangularPrism::new(height, radius))
}
pub fn translate(obj: Arc<Raymarchable>, trans: [f64; 3]) -> Arc<Raymarchable> {
    Arc::new(Translate::new(vec3d(trans), obj))
}
pub fn transform(obj: Arc<Raymarchable>, trans: Mat3d) -> Arc<Raymarchable> {
    Arc::new(Transform::new(trans, obj))
}
pub fn repeat(obj: Arc<Raymarchable>, modulus: [f64; 3]) -> Arc<Raymarchable> {
    Arc::new(Repeat::new(obj, vec3d(modulus)))
}
pub fn hollow(obj: Arc<Raymarchable>) -> Arc<Raymarchable> {
    Arc::new(Hollow::new(obj))
}
pub fn sierpinski(iterations: u32, scale: f64) -> Arc<Raymarchable> {
    Arc::new(Sierpinski::new(iterations, scale))
}

/* Materials */
pub fn solid_colour(col: [f32; 3]) -> Arc<Material> {
    Arc::new(SolidColour::new(colour(col)))
}
pub fn mirror() -> Arc<Material> {
    Arc::new(Mirror {})
}
pub fn normal(obj: Arc<Raymarchable>) -> Arc<Material> {
    Arc::new(NormalColour::new(obj))
}

/* Lights */
pub fn ambient(col: [f32; 3]) -> Arc<Light> {
    Arc::new(AmbientLight::new(colour(col)))
}
pub fn directional(dir: [f64; 3]) -> Arc<Light> {
    Arc::new(DirectionalLight::new(vec3d(dir)))
}
pub fn fuzzy_directional(dir: [f64; 3], fuzziness: f64) -> Arc<Light> {
    Arc::new(FuzzyDirectionalLight::new(vec3d(dir), fuzziness))
}
pub fn tint(light: Arc<Light>, col: [f32; 3]) -> Arc<Light> {
    Arc::new(Tint::new(light, colour(col)))
}
pub fn point_light(pos: [f64; 3], power: f32) -> Arc<Light> {
    Arc::new(PointLight::new(vec3d(pos), power))
}

/* Adapter Methods */
pub fn deg2rad(deg: f64) -> f64 {
    deg * (PI / 180.0)
}
pub fn rotate_xyz(angle_x: f64, angle_y: f64, angle_z: f64) -> Mat3d {
    let (sinx, cosx) = angle_x.sin_cos();
    let (siny, cosy) = angle_y.sin_cos();
    let (sinz, cosz) = angle_z.sin_cos();

    let x = Mat3d::new(1.0, 0.0, 0.0, 0.0, cosx, -sinx, 0.0, sinx, cosx);
    let y = Mat3d::new(cosy, 0.0, siny, 0.0, 1.0, 0.0, -siny, 0.0, cosy);
    let z = Mat3d::new(cosz, -sinz, 0.0, sinz, cosz, 0.0, 0.0, 0.0, 1.0);

    return x * y * z;
}
