
use vec::*;
use intersection::*;

pub trait Material {
	fn surface_colour(&self, isect : Intersection) -> Colour;
}
