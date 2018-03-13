use lib::*;
use lib::object::Raymarchable;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Transform<T: Raymarchable> 
{
    mat: Mat3d,
    inv: Mat3d,
    obj: T,
}

impl<T: Raymarchable + Sized> Transform<T> {
    pub fn new(mat: Mat3d, obj: T) -> Self {
        Self {
            obj: obj,
            mat: mat,
            inv: mat.inverse(),
        }
    }
}

impl<'de, T: Raymarchable> Raymarchable for Transform<T>
	where T: Serialize + Deserialize<'de>
{
    fn distance(&self, point: Vec3d) -> f64 {
        self.obj.distance(self.inv * point)
    }
    fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
        //self.obj.normal_at(self.mat * point, dir)
        self.mat
            * self.obj
                .normal_at(self.inv * point, self.inv * dir)
                .normalize()
    }
}
