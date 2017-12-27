
use lib::*;

pub trait Light {
	fn illumination(isect : &Intersection) -> Colour;
}
