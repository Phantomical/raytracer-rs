use vec::*;

use object::Raymarchable;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Rotate<T: Raymarchable> {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    mat: Mat3d,
    //inv : Basis3<f64>,
    obj: T,
}

type_serialization_decl!("rotate");

impl<T: Raymarchable> Rotate<T> {
    pub fn new(obj: T, mat: Mat3d) -> Self {
        Self {
            obj,
            mat,
            //inv: mat.inverse().expect("Rotation matrix was not invertible")
            tag: (),
        }
    }
}

impl<'de, T: Raymarchable> Raymarchable for Rotate<T>
where
    T: Serialize + Deserialize<'de>,
{
    fn distance(&self, point: Vec3d) -> f64 {
        self.obj.distance(self.mat * point)
    }
}
