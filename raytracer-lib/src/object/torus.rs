use lib::*;
use lib::object::*;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Torus {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    inner_radius: f64,
    outer_radius: f64,
}

type_serialization_decl!("torus");

impl Torus {
    pub fn new(inner_radius: f64, outer_radius: f64) -> Torus {
        return Self {
            inner_radius,
            outer_radius,
            tag: (),
        };
    }
}

impl Raymarchable for Torus {
    fn distance(&self, point: Vec3d) -> f64 {
        let q = vec2(point.xz().length() - self.inner_radius, point.y);
        return q.length() - self.outer_radius;
    }
}
