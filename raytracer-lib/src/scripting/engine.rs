#![allow(dead_code)]
use rhai::{Engine, RegisterFn};

use std::ops::*;
use std::rc::Rc;

use lib::*;
use lib::object::Raymarchable;
use lib::material::Material;

use scripting::swizzle;

fn add<B, A: Add<B>>(x: A, y: B) -> A::Output {
    x + y
}
fn sub<B, A: Sub<B>>(x: A, y: B) -> A::Output {
    x - y
}
fn mul<B, A: Mul<B>>(x: A, y: B) -> A::Output {
    x * y
}
fn div<B, A: Div<B>>(x: A, y: B) -> A::Output {
    x / y
}
fn neg<T: Neg>(x: T) -> <T as Neg>::Output {
    -x
}

macro_rules! bin_op {
	($engine:expr, $x:expr, $op:expr, $t1:ty, $t2:ty, $out:ty) => {
		$engine.register_fn($x, ($op as fn(x: $t1, y: $t2) -> $out));
	}
}

macro_rules! reg_bin_ops {
	($engine:expr, $vec:ty, $elem:ty) => {
		bin_op!($engine, "+", add, $vec, $vec, $vec);
		bin_op!($engine, "-", sub, $vec, $vec, $vec);
		bin_op!($engine, "*", mul, $vec, $vec, $vec);

		bin_op!($engine, "+", add, $vec, $elem, $vec);
		bin_op!($engine, "-", sub, $vec, $elem, $vec);
		bin_op!($engine, "*", mul, $vec, $elem, $vec);
		bin_op!($engine, "/", div, $vec, $elem, $vec);

		bin_op!($engine, "+", add, $elem, $vec, $vec);
		bin_op!($engine, "-", sub, $elem, $vec, $vec);
		bin_op!($engine, "*", mul, $elem, $vec, $vec);
	}
}

pub fn register_vec_fns(engine: &mut Engine) {
    /* Get and Set for Vec3d elements */
    fn get_x_vec3(v: &mut Vec3d) -> f64 {
        v.x
    }
    fn get_y_vec3(v: &mut Vec3d) -> f64 {
        v.y
    }
    fn get_z_vec3(v: &mut Vec3d) -> f64 {
        v.z
    }

    fn set_x_vec3(v: &mut Vec3d, val: f64) {
        v.x = val;
    }
    fn set_y_vec3(v: &mut Vec3d, val: f64) {
        v.y = val;
    }
    fn set_z_vec3(v: &mut Vec3d, val: f64) {
        v.z = val;
    }

    /* Get and Set for Vec3d elements */
    fn get_x_vec2(v: &mut Vec3d) -> f64 {
        v.x
    }
    fn get_y_vec2(v: &mut Vec3d) -> f64 {
        v.y
    }

    fn set_x_vec2(v: &mut Vec3d, val: f64) {
        v.x = val;
    }
    fn set_y_vec2(v: &mut Vec3d, val: f64) {
        v.y = val;
    }

    /* Get and Set for Colour elements (renamed to r, g, b) */
    fn get_x_colour(v: &mut Vec3d) -> f64 {
        v.x
    }
    fn get_y_colour(v: &mut Vec3d) -> f64 {
        v.y
    }
    fn get_z_colour(v: &mut Vec3d) -> f64 {
        v.z
    }

    fn set_x_colour(v: &mut Vec3d, val: f64) {
        v.x = val;
    }
    fn set_y_colour(v: &mut Vec3d, val: f64) {
        v.y = val;
    }
    fn set_z_colour(v: &mut Vec3d, val: f64) {
        v.z = val;
    }

    engine.register_type::<Vec3d>();
    engine.register_type::<Vec2d>();
    engine.register_type::<Colour>();

    reg_bin_ops!(engine, Vec3d, f64);
    reg_bin_ops!(engine, Vec2d, f64);
    reg_bin_ops!(engine, Colour, f32);

    engine.register_get_set("x", get_x_vec3, set_x_vec3);
    engine.register_get_set("y", get_y_vec3, set_y_vec3);
    engine.register_get_set("z", get_z_vec3, set_z_vec3);

    engine.register_get_set("x", get_x_vec2, set_x_vec2);
    engine.register_get_set("y", get_y_vec2, set_y_vec2);

    /* Rename x,y,z on Colour to be r,g,b for ease of use */
    engine.register_get_set("r", get_x_colour, set_x_colour);
    engine.register_get_set("g", get_y_colour, set_y_colour);
    engine.register_get_set("b", get_z_colour, set_z_colour);

    engine.register_fn("-", neg as fn(x: Vec3d) -> Vec3d);
    engine.register_fn("-", neg as fn(x: Vec2d) -> Vec2d);
    engine.register_fn("-", neg as fn(x: Colour) -> Colour);

    engine.register_fn("vec3", vec3 as fn(x: f64, y: f64, z: f64) -> Vec3d);
    engine.register_fn("vec2", vec2 as fn(x: f64, y: f64) -> Vec2d);
    engine.register_fn("colour", vec3 as fn(x: f32, y: f32, z: f32) -> Colour);
}
pub fn register_ray_fns(engine: &mut Engine) {
    fn get_origin(ray: &mut Ray) -> Vec3d {
        ray.origin
    }
    fn get_direction(ray: &mut Ray) -> Vec3d {
        ray.direction
    }

    engine.register_type::<Ray>();

    engine.register_get("origin", get_origin);
    engine.register_get("direction", get_direction);
}
pub fn register_isect_fns(engine: &mut Engine) {
    fn get_point(isect: &mut Intersection) -> Vec3d {
        isect.point
    }
    fn get_ray(isect: &mut Intersection) -> Ray {
        isect.ray
    }
    fn get_normal(isect: &mut Intersection) -> Vec3d {
        isect.normal
    }
    fn get_object(isect: &mut Intersection) -> Rc<Raymarchable> {
        Rc::clone(&isect.object.object)
    }
    fn get_material(isect: &mut Intersection) -> Rc<Material> {
        Rc::clone(&isect.object.material)
    }

    engine.register_type::<Intersection>();

    engine.register_get("point", get_point);
    engine.register_get("ray", get_ray);
    engine.register_get("normal", get_normal);
    engine.register_get("object", get_object);
    engine.register_get("material", get_material);
}
pub fn register_raymarchable_fns(engine: &mut Engine) {
    fn distance(ptr: &mut Rc<Raymarchable>, point: Vec3d) -> f64 {
        ptr.distance(point)
    }
    fn normal_at(ptr: &mut Rc<Raymarchable>, point: Vec3d, dir: Vec3d) -> Vec3d {
        ptr.normal_at(point, dir)
    }

    engine.register_type::<Rc<Raymarchable>>();

    engine.register_fn("distance", distance);
    engine.register_fn("normal_at", normal_at);
}
pub fn register_material_fns(engine: &mut Engine) {
    fn base_colour(ptr: &mut Rc<Material>, isect: &Intersection) -> Colour {
        ptr.base_colour(isect)
    }
    fn roughness(ptr: &mut Rc<Material>, isect: &Intersection) -> f32 {
        ptr.roughness(isect)
    }
    fn reflectivity(ptr: &mut Rc<Material>, isect: &Intersection) -> f32 {
        ptr.reflectivity(isect)
    }

    engine.register_type::<Rc<Material>>();

    engine.register_fn("base_colour", base_colour);
    engine.register_fn("roughness", roughness);
    engine.register_fn("reflectivity", reflectivity);
}

pub fn build() -> Engine {
    let mut engine = Engine::new();

    Engine::register_default_lib(&mut engine);

    register_vec_fns(&mut engine);
    register_ray_fns(&mut engine);
    register_isect_fns(&mut engine);
    register_raymarchable_fns(&mut engine);
    register_material_fns(&mut engine);
    swizzle::register_swizzle_vec3d_fns(&mut engine);
    swizzle::register_swizzle_vec2d_fns(&mut engine);

    engine
}
