use lib::*;
use lib::object::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct HexagonalPrism {
	#[serde(with = "tag")]
	#[serde(rename = "type")]
	#[serde(skip_deserializing)]
	tag: (),

    radius: f64,
    height: f64,
}

type_serialization_decl!("hexagonal_prism");

impl HexagonalPrism {
    pub fn new(height: f64, radius: f64) -> Self {
        return Self { radius, height, tag: () };
    }
}

impl Raymarchable for HexagonalPrism {
    fn distance(&self, p: Vec3d) -> f64 {
        let q = abs(p);
        return (q.z - self.height).max((q.x * 0.866025 + q.y * 0.5).max(q.y) - self.radius);
    }
}
