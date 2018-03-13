use vec::*;
use material::Material;
use lib::Intersection;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Mirror {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),
}

type_serialization_decl!("mirror");

impl Mirror {
    pub fn new() -> Self {
        Self { tag: () }
    }
}

impl Material for Mirror {
    fn base_colour(&self, _isect: &Intersection) -> Colour {
        return Colour::zero();
    }
    fn roughness(&self, _isect: &Intersection) -> f32 {
        return 0.0;
    }
    fn reflectivity(&self, _isect: &Intersection) -> f32 {
        return 1.0;
    }
}
