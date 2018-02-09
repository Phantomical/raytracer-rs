
use lib::{Vec3d};
use lib::object::{Raymarchable, normal_finite_difference};

use scripting::*;
use cacheable::*;

pub struct ScriptedRaymarchable(Script);

impl Raymarchable for ScriptedRaymarchable {
	fn distance(&self, point: Vec3d) -> f64 {
		self.0.call_distance(point)
			.expect("An error occurred while trying to call a distance function within a script")
	}

	fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
		let result = self.0.call_normal(point, dir);

		if result.is_ok() {
			return result.unwrap();
		}
		else {
			let err = result.unwrap_err();

			return match err {
				FunctionCallError::FunctionNotFound => {
					normal_finite_difference(self, point)
				},
				FunctionCallError::FunctionArgMismatch => {
					panic!("Error: normal function in a script had incorrect arguments")
				}
				_ => panic!("Error: an error occurred while executing the normal function")
			};
		}
	}
}

impl Cacheable<ScriptedRaymarchable> for CachedScript {
	fn cached(&self) -> ScriptedRaymarchable {
		ScriptedRaymarchable(Script::new(self))
	}
}

