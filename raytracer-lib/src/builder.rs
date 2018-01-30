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
    return Vec2d::new(vals);
}
pub fn vec3d(vals: [f64; 3]) -> Vec3d {
    return Vec3d::new(vals);
}
pub fn colour(vals: [f32; 3]) -> Colour {
    return Colour::new(vals);
}

/* Objects */
pub fn sphere(pos: [f64; 3], radius: f64) -> Sphere {
    Sphere::new(vec3d(pos), radius)
}
pub fn box_obj(bounds: [f64; 3]) -> BoxObj {
    BoxObj::new(vec3d(bounds))
}
pub fn cone(radius: f64, height: f64) -> Cone {
    Cone::new(radius, height)
}
pub fn cylinder(c: [f64; 3]) -> Cylinder {
    Cylinder::new(vec3d(c))
}
pub fn hexagonal_prism(height: f64, radius: f64) -> HexagonalPrism {
    HexagonalPrism::new(height, radius)
}
pub fn plane(normal: [f64; 3], point: [f64; 3]) ->  Plane {
    Plane::new(vec3d(normal), vec3d(point))
}
pub fn torus(inner: f64, outer: f64) -> Torus {
    Torus::new(inner, outer)
}
pub fn triangular_prism(height: f64, radius: f64) -> TriangularPrism {
    TriangularPrism::new(height, radius)
}
pub fn translate<T>(obj: T, trans: [f64; 3]) -> Translate<T>
	where T: Raymarchable
{
    Translate::new(vec3d(trans), obj)
}
pub fn transform<T>(obj: T, trans: Mat3d) -> Transform<T>
	where T: Raymarchable
{
    Transform::new(trans, obj)
}
pub fn repeat<T>(obj: T, modulus: [f64; 3]) -> Repeat<T>
	where T: Raymarchable + Copy
{
    Repeat::new(obj, vec3d(modulus))
}
pub fn hollow<T>(obj: T) -> Hollow<T>
	where T: Raymarchable + Copy
{
    Hollow::new(obj)
}
pub fn sierpinski(iterations: u32, scale: f64) -> Sierpinski {
    Sierpinski::new(iterations, scale)
}
pub fn rotate<T>(obj: T, xyz: Mat3d) -> Rotate<T>
	where T: Raymarchable + Copy
{
    Rotate::new(obj, xyz)
}
pub fn mandelbulb(iterations: usize, power: i32) -> Mandelbulb {
    Mandelbulb::new(iterations, power)
}

/* Materials */
pub fn solid_colour(col: [f32; 3]) -> SolidColour {
	SolidColour::new(colour(col))
}
pub fn mirror() -> Mirror {
    Mirror {}
}
pub fn normal() -> NormalColour {
    NormalColour::new()
}
pub fn mandelbulb_orbit_trap(iterations: usize, power: i32) -> MandelbulbOrbitTrap {
    MandelbulbOrbitTrap::new(iterations, power)
}

/* Lights */
pub fn ambient(col: [f32; 3]) -> AmbientLight {
    AmbientLight::new(colour(col))
}
pub fn directional(dir: [f64; 3]) -> DirectionalLight {
    DirectionalLight::new(vec3d(dir))
}
pub fn fuzzy_directional(dir: [f64; 3], fuzziness: f64, rays: usize) -> FuzzyDirectionalLight {
    FuzzyDirectionalLight::new(vec3d(dir), fuzziness, rays)
}
pub fn tint<T>(light: T, col: [f32; 3]) -> Tint<T>
	where T: Light
{
    Tint::new(light, colour(col))
}
pub fn point_light(pos: [f64; 3], power: f32) -> PointLight {
    PointLight::new(vec3d(pos), power)
}

/* Adapter Methods */
pub fn deg2rad(deg: f64) -> f64 {
    deg * (PI / 180.0)
}
pub fn rotate_xyz(angle_x: f64, angle_y: f64, angle_z: f64) -> Mat3d {
    let (sinx, cosx) = angle_x.sin_cos();
    let (siny, cosy) = angle_y.sin_cos();
    let (sinz, cosz) = angle_z.sin_cos();

    let x = Mat3d::new([1.0, 0.0, 0.0, 0.0, cosx, -sinx, 0.0, sinx, cosx]);
    let y = Mat3d::new([cosy, 0.0, siny, 0.0, 1.0, 0.0, -siny, 0.0, cosy]);
    let z = Mat3d::new([cosz, -sinz, 0.0, sinz, cosz, 0.0, 0.0, 0.0, 1.0]);

    return x * y * z;
}
