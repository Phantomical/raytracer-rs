
use lib::{Intersection, Colour, Ray};

pub trait Light: Sync + Send {
	fn illumination(&self, isect : &Intersection) -> Colour;
	// Returns a (ray, distance) pair
	fn shadow_ray(&self, isect : &Intersection) -> (Ray, f64);
}
