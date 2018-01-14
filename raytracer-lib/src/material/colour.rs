use vec::{Colour, Vec3d};
use material::Material;

pub struct SolidColour {
    pub colour: Colour,
}

impl SolidColour {
    pub fn new(colour: Colour) -> Self {
        return Self { colour: colour };
    }
}

impl Material for SolidColour {
    fn base_colour(&self, _point: Vec3d) -> Colour {
        return self.colour;
    }

    fn roughness(&self, _point: Vec3d) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _point: Vec3d) -> f32 {
        return 0.0;
    }
}
