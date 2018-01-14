use lib::{Colour, Intersection, Ray};

pub trait Light: Sync + Send {
    fn illumination(&self, isect: &Intersection) -> Colour;
    // Returns a (ray, distance) pair
    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>>;
}
