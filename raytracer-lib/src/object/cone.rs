use lib::*;
use lib::object::*;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Cone {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    c: Vec2d,
}

type_serialization_decl!("cone");

impl Cone {
    pub fn new(radius: f64, height: f64) -> Self {
        Self {
            c: vec2(radius, height).normalize(),
            tag: (),
        }
    }
}

impl Raymarchable for Cone {
    fn distance(&self, p: Vec3d) -> f64 {
        let q = p.xy().length();
        return dot(self.c, vec2(q, p.z));
    }
}
