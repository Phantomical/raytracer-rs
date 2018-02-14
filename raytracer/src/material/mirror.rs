use vec::*;
use material::Material;
use lib::Intersection;

#[derive(Copy, Clone)]
pub struct Mirror {}

impl Material for Mirror {
    fn base_colour(&self, _isect: &Intersect) -> Colour {
        return Colour::zero();
    }
    fn roughness(&self, _isect: &Intersect) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _isect: &Intersection) -> f32 {
        return 1.0;
    }
}
