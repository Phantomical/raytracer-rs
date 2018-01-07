
use lib::*;
use light::*;
use std::iter::once;

const DIRECTIONAL_DISTANCE : f64 = 1.0e10;

#[derive(Deserialize)]
pub struct DirectionalLight {
	direction : Vec3d
}

impl Light for DirectionalLight {
	fn illumination(&self, isect : &Intersection) -> Colour {
		let mult = (dot(isect.normal, -self.direction) as f32).abs();
		return Colour::new(mult, mult, mult);
	}

	fn shadow_rays(&self, isect : &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
		return Box::new(once((
			Ray::new(isect.point + isect.normal * 0.0001, -self.direction), 
			DIRECTIONAL_DISTANCE
		)));
	}
}

impl DirectionalLight {
	pub fn new(direction : Vec3d) -> DirectionalLight{
		return DirectionalLight {
			direction: direction
		};
	}
}

