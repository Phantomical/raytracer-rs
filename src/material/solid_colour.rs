
use tracer::*;
use material::traits::*;

pub struct SolidColour {
	pub colour : Colour
}

impl Material for SolidColour {
	fn surface_colour(&self, _isect : Intersection) -> Colour {
		return self.colour;
	}
}
