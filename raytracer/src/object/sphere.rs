
use prelude::Object;
use vec::{Vec3d, normalize, length};
use array_vec::ArrayVec3;
use ndarray::{Array, IxDyn};

#[derive(Copy, Clone)]
pub struct Sphere {
	centre: Vec3d,
	radius: f64
}

impl Sphere {
    pub fn new(centre: Vec3d, radius: f64) -> Sphere {
        return Sphere {
            centre: centre,
            radius: radius,
        };
    }
}

impl Object for Sphere {
	fn distance(&self, points: &ArrayVec3) -> Array<f64, IxDyn> {
		length(points - ArrayVec3::from(self.centre)) - self.radius
	}
	fn normal_at(&self, points: &ArrayVec3) -> ArrayVec3 {
		normalize(points - ArrayVec3::from(self.centre))
	}
}
