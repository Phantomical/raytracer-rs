
use lib::*;
use lib::light::*;

pub struct AmbientLight {
	colour : Colour
}

impl AmbientLight {
	fn new(colour : Colour) -> AmbientLight {
		AmbientLight {
			colour: colour
		}
	}
}

impl Light for AmbientLight {
	fn illumination(&self, isect : &Intersection) -> Colour {
		return self.colour;
	}

	fn shadow_ray(&self, isect : &Intersection) -> (Ray, f64) {
		return (Ray{..Default::default()}, 0.0);
	}
}
