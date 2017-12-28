
use lib::*;
use light::*;

const DIRECTIONAL_DISTANCE : f64 = 1.0e10;

pub struct DirectionalLight {
	direction : Vec3d
}

impl Light for DirectionalLight {
	fn illumination(&self, isect : &Intersection) -> Colour {
		let mult = dot(isect.normal, self.direction) as f32;
		return Colour::new(mult, mult, mult);
	}

	fn shadow_ray(&self, isect : &Intersection) -> (Ray, f64) {
		return (
			Ray::new(isect.point, -self.direction), 
			DIRECTIONAL_DISTANCE
		);
	}
}

impl DirectionalLight {
	pub fn new(direction : Vec3d) -> DirectionalLight{
		return DirectionalLight {
			direction: direction
		};
	}
}

