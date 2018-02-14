
use vec::Colour;
use types::Intersect;

pub trait Material: Send + Sync {
	fn base_colour(&self, isect: &Intersect) -> Colour;
}