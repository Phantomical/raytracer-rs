use lib::*;
use lib::object::*;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct TriangularPrism {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    height: f64,
    radius: f64,
}

type_serialization_decl!("triangular_prism");

impl TriangularPrism {
    pub fn new(height: f64, radius: f64) -> Self {
        Self {
            height,
            radius,
            tag: (),
        }
    }
}

impl Raymarchable for TriangularPrism {
    fn distance(&self, p: Vec3d) -> f64 {
        let q = abs(p);
        return (q.z - self.height).max((q.x * 0.866025 + p.y * 0.5).max(-p.y) - self.radius * 0.5);
    }
}
