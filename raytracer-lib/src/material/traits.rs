use vec::*;
use lib::Intersection;
use cacheable::Cacheable;
use std::rc::Rc;

use erased_serde::Serialize;

pub trait Material: Serialize {
    fn base_colour(&self, isect: &Intersection) -> Colour;

    fn roughness(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
}

serialize_trait_object!(Material);

impl<T> Cacheable<Rc<Material>> for T
where
    T: Cacheable<T> + Material + 'static,
{
    fn cached(&self) -> Rc<Material> {
        Rc::new(self.cached())
    }
}
