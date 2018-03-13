use lib::*;
use lib::light::*;
use std::iter::empty;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct AmbientLight {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    colour: Colour,
}

type_serialization_decl!("ambient");

impl AmbientLight {
    pub fn new(colour: Colour) -> AmbientLight {
        AmbientLight {
            colour: colour,
            tag: (),
        }
    }
}

impl Light for AmbientLight {
    fn illumination(&self, _isect: &Intersection) -> Colour {
        return self.colour;
    }

    fn shadow_rays(&self, _isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        return Box::new(empty());
    }
}
