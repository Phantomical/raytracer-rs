use lib::{Colour, Intersection, Ray};
use erased_serde::Serialize;

pub trait Light: Sync + Send + Serialize {
    fn illumination(&self, isect: &Intersection) -> Colour;
    // Returns a (ray, distance) pair
    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>>;
}

serialize_trait_object!(Light);
