
use lib::*;
use light::*;

pub struct DirectionalLight {
	direction : Vec3d
}

impl Light for DirectionalLight {
	fn illumination(&self, isect : &Intersection) -> Colour {
		let mult = dot(isect.normal, self.direction) as f32;
		return Colour::new(mult, mult, mult);
	}
}

impl DirectionalLight {
	pub fn new(direction : Vec3d) -> DirectionalLight{
		return DirectionalLight {
			direction: direction
		};
	}
}

