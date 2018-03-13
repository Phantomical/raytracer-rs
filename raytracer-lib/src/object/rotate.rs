use vec::*;

use object::Raymarchable;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Rotate<T: Raymarchable> {
    mat: Mat3d,
    //inv : Basis3<f64>,
    obj: T,
}

impl<T: Raymarchable> Rotate<T> {
    pub fn new(obj: T, mat: Mat3d) -> Self {
        Self {
            obj,
            mat,
            //inv: mat.inverse().expect("Rotation matrix was not invertible")
        }
    }
}

impl<'de, T: Raymarchable> Raymarchable for Rotate<T> 
	where T: Serialize + Deserialize<'de>
{
    fn distance(&self, point: Vec3d) -> f64 {
        self.obj
            .distance(self.mat * point)
    }
}
