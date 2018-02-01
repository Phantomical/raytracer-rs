
use rhai::Engine;
use lib::object::{Raymarchable, Analytical, IFS};
use vec::Vec3d;

mod engine {
	use rhai::{Engine, RegisterFn};
	use std::ops::*;
	use std::cell::*;
	use std::rc::*;
	use vec::*;

	fn add<B, A: Add<B>>(x: A, y: B) -> A::Output { x + y }
	fn sub<B, A: Sub<B>>(x: A, y: B) -> A::Output { x - y }
	fn mul<B, A: Mul<B>>(x: A, y: B) -> A::Output { x * y }
	fn div<B, A: Div<B>>(x: A, y: B) -> A::Output { x / y }
    fn neg<T: Neg>(x: T) -> <T as Neg>::Output    { -x }

	fn get_x_vec3(v: &mut Vec3d) -> f64 { v.x }
	fn get_y_vec3(v: &mut Vec3d) -> f64 { v.y }
	fn get_z_vec3(v: &mut Vec3d) -> f64 { v.z }

	fn set_x_vec3(v: &mut Vec3d, val: f64) { v.x = val; }
	fn set_y_vec3(v: &mut Vec3d, val: f64) { v.y = val; }
	fn set_z_vec3(v: &mut Vec3d, val: f64) { v.z = val; }

	macro_rules! bin_op {
		($engine:expr, $x:expr, $op:expr, $t1:ty, $t2:ty, $out:ty) => {
			$engine.register_fn($x, ($op as fn(x: $t1, y: $t2) -> $out));
		}
	}
    
    fn build_impl() -> Engine {
		let mut engine = Engine::new();

		Engine::register_default_lib(&mut engine);

		engine.register_type::<Vec3d>();

		bin_op!(engine, "+", add, Vec3d, Vec3d, Vec3d);
		bin_op!(engine, "-", sub, Vec3d, Vec3d, Vec3d);
		bin_op!(engine, "*", mul, Vec3d, Vec3d, Vec3d);
		
		bin_op!(engine, "+", add, Vec3d, f64, Vec3d);
		bin_op!(engine, "-", sub, Vec3d, f64, Vec3d);
		bin_op!(engine, "*", mul, Vec3d, f64, Vec3d);
		bin_op!(engine, "/", div, Vec3d, f64, Vec3d);
		
		bin_op!(engine, "+", add, f64, Vec3d, Vec3d);
		bin_op!(engine, "-", sub, f64, Vec3d, Vec3d);
		bin_op!(engine, "*", mul, f64, Vec3d, Vec3d);

		engine.register_get_set("x", get_x_vec3, set_x_vec3);
		engine.register_get_set("y", get_y_vec3, set_y_vec3);
		engine.register_get_set("z", get_z_vec3, set_z_vec3);

		engine.register_fn("-", neg as fn(x: Vec3d) -> Vec3d);

		engine.register_fn("vec3", vec3 as fn(x: f64, y: f64, z: f64) -> Vec3d);

		engine
	}
    
	thread_local! {
		static ENGINE: RefCell<Option<Rc<Engine>>> = RefCell::new(None);
	}

	pub fn build() -> Engine {
		ENGINE.with(|e| {
			if let Some(ref rc) = *e.borrow() {
				rc.deref().clone()
			}
			else {
				let engine = build_impl();
				*e.borrow_mut() = Some(Rc::new(engine.clone()));
				engine
			}
		})
	}
}

#[derive(Clone)]
struct Temp {
	eng: Engine
}

pub struct ScriptedRaymarchableObject {
	source: String
}

impl ScriptedRaymarchableObject {
	pub fn new(source: &str) -> Self {
		Self {
			source: source.to_string()
		}
	}
}

impl Raymarchable for ScriptedRaymarchableObject {
	fn distance(&self, point: Vec3d) -> f64 {
		let engine = engine::build();

		unimplemented!();
	}
}