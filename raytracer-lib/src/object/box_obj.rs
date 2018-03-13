use lib::*;
use lib::object::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BoxObj {
	#[serde(with = "tag")]
	#[serde(rename = "type")]
	#[serde(skip_deserializing)]
	tag: (),

    pub bounds: Vec3d,
}

type_serialization_decl!("box");

impl BoxObj {
    pub fn new(bounds: Vec3d) -> Self {
        Self { bounds, tag: () }
    }
}

impl Raymarchable for BoxObj {
    fn distance(&self, point: Vec3d) -> f64 {
        let d = abs(point) - self.bounds;

        return d.x.max(d.y.max(d.z)).min(0.0) + length(max(d, Vec3d::zero()));
    }
}
