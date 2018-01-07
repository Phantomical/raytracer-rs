
use lib::*;
use lib::light::*;
use std::iter::empty;

pub struct AmbientLight {
	colour : Colour
}

impl AmbientLight {
	pub fn new(colour : Colour) -> AmbientLight {
		AmbientLight {
			colour: colour
		}
	}
}

impl Light for AmbientLight {
	fn illumination(&self, _isect : &Intersection) -> Colour {
		return self.colour;
	}

	fn shadow_rays(&self, _isect : &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
		return Box::new(empty());
	}
}
