use lib::*;
use lib::object::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Translate<T: Raymarchable> {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    position: Vec3d,
    subobj: T,
}

type_serialization_decl!("translate");

impl<T: Raymarchable + Sized> Translate<T> {
    pub fn new(pos: Vec3d, obj: T) -> Self {
        return Self {
            position: pos,
            subobj: obj,
            tag: (),
        };
    }
}

impl<'de, T> Raymarchable for Translate<T>
where
    T: Raymarchable + Sized + Serialize + Deserialize<'de>,
{
    fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
        return self.subobj.normal_at(point - self.position, dir);
    }

    fn distance(&self, point: Vec3d) -> f64 {
        return self.subobj.distance(point - self.position);
    }
}

pub fn translate<T>(obj: T, pos: Vec3d) -> Translate<T>
where
    T: Raymarchable + Sized,
{
    return Translate::new(pos, obj);
}
