
use lib::{Intersection, Colour};

pub trait Light {
	fn illumination(&self, isect : &Intersection) -> Colour;
}
