use vec::Colour;
use prelude::Material;
use types::Intersect;

#[derive(Copy, Clone)]
pub struct SolidColour {
    pub colour: Colour,
}

impl SolidColour {
    pub fn new(colour: Colour) -> Self {
        return Self { colour: colour };
    }
}

impl Material for SolidColour {
    fn base_colour(&self, _isect: &Intersect) -> Colour {
        return self.colour;
    }
}
