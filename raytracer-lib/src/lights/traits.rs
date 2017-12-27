
use lib::{Intersection, Colour};

pub trait Light {
	fn illumination(isect : &Intersection) -> Colour;
}
