use lib::*;
use lib::object::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Cylinder {
	#[serde(with = "tag")]
	#[serde(rename = "type")]
	#[serde(skip_deserializing)]
	tag: (),

    c: Vec3d,
}

type_serialization_decl!("cylinder");

impl Cylinder {
    pub fn new(c: Vec3d) -> Self {
        Self { c, tag: () }
    }
}

impl Raymarchable for Cylinder {
    fn distance(&self, p: Vec3d) -> f64 {
        return (p.xz() - self.c.xy()).length() - self.c.z;
    }
}
