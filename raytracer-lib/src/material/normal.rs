use lib::*;
use lib::material::Material;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct NormalColour {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),
}

type_serialization_decl!("normal_colour");

impl NormalColour {
    pub fn new() -> Self {
        Self { tag: () }
    }
}

impl Material for NormalColour {
    fn base_colour(&self, isect: &Intersection) -> Colour {
        let normal = isect.normal;
        vec3(
            normal.x.abs() as f32,
            normal.y.abs() as f32,
            normal.z.abs() as f32,
        )
    }
}
