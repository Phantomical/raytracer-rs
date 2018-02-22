
use vec::Colour;
use types::{Ray, Intersect};

pub trait Light {
	fn illumination(&self, isect: &Intersect) -> Colour;

	fn shadow_rays(&self, isect: &Intersect) -> Ray;
}
