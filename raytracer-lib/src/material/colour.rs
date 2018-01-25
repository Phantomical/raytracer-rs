use vec::Colour;
use material::Material;
use lib::Intersection;

pub struct SolidColour {
    pub colour: Colour,
}

impl SolidColour {
    pub fn new(colour: Colour) -> Self {
        return Self { colour: colour };
    }
}

impl Material for SolidColour {
    fn base_colour(&self, _isect: &Intersection) -> Colour {
        return self.colour;
    }

    fn roughness(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
}
