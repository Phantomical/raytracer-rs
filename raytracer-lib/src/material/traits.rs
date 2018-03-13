use vec::*;
use lib::Intersection;

use erased_serde::Serialize;

pub trait Material: Sync + Send + Serialize {
    fn base_colour(&self, isect: &Intersection) -> Colour;

    fn roughness(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
}

serialize_trait_object!(Material);
