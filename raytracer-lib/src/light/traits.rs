use lib::{Colour, Intersection, Ray};
use cacheable::Cacheable;
use std::rc::Rc;

pub trait Light: Sync + Send {
    fn illumination(&self, isect: &Intersection) -> Colour;
    // Returns a (ray, distance) pair
    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>>;
}


impl<T> Cacheable<Rc<Light>> for T
	where T: Cacheable<T> + Light + 'static
{
	fn cached(&self) -> Rc<Light> {
		Rc::new(self.cached())
	}
}
