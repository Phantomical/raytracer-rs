use vec::Colour;
use material::Material;
use lib::Intersection;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SolidColour {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    pub colour: Colour,
}

type_serialization_decl!("colour");

impl SolidColour {
    pub fn new(colour: Colour) -> Self {
        return Self { colour: colour, tag:() };
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
