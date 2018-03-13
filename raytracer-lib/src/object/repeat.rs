use lib::*;
use lib::object::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Repeat<T: Raymarchable> {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    modulus: Vec3d,
    obj: T,
}

type_serialization_decl!("repeat");

impl<T: Raymarchable> Repeat<T> {
    pub fn new(obj: T, modulus: Vec3d) -> Self {
        Self {
            obj,
            modulus,
            tag: (),
        }
    }
}

fn modulus(v: Vec3d, m: Vec3d) -> Vec3d {
    vec3(
        if m.x == 0.0 { v.x } else { v.x % m.x },
        if m.y == 0.0 { v.y } else { v.y % m.y },
        if m.z == 0.0 { v.z } else { v.z % m.z },
    )
}

impl<'de, T: Raymarchable> Raymarchable for Repeat<T>
where
    T: Serialize + Deserialize<'de>,
{
    fn distance(&self, point: Vec3d) -> f64 {
        self.obj
            .distance(modulus(point, self.modulus) - (self.modulus * 0.5))
    }

    fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
        self.obj
            .normal_at(modulus(point, self.modulus) - (self.modulus * 0.5), dir)
    }
}
