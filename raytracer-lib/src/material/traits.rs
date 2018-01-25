use vec::*;
use lib::Intersection;

pub trait Material: Sync + Send {
    fn base_colour(&self, isect: &Intersection) -> Colour;

    fn roughness(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
}
