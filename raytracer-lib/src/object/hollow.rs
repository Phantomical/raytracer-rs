use lib::*;
use lib::object::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Hollow<T: Raymarchable> {
	#[serde(with = "tag")]
	#[serde(rename = "type")]
	#[serde(skip_deserializing)]
	tag: (),

    obj: T,
}

type_serialization_decl!("hollow");

impl<T: Raymarchable> Hollow<T> {
    pub fn new(obj: T) -> Self {
        Self { obj, tag: () }
    }
}

impl<'de, T: Raymarchable> Raymarchable for Hollow<T>
	where T: Serialize + Deserialize<'de>
{
    fn distance(&self, point: Vec3d) -> f64 {
        self.obj.distance(point).abs()
    }

    fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
        let dist = self.distance(point);

        self.obj.normal_at(point, dir) * if dist < 0.0 { -1.0 } else { 1.0 }
    }
}
