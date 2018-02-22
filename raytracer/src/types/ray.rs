
use array_vec::ArrayVec3;

use ndarray::{Array, IxDyn};

#[derive(Clone)]
pub struct Ray {
	pub points: ArrayVec3,
	pub dirs: ArrayVec3
}

impl Ray {
	pub fn new(points: ArrayVec3, dirs: ArrayVec3) -> Self {
		Self { points, dirs }
	}

	pub fn points_at(&self, distance: &Array<f64, IxDyn>) -> ArrayVec3 {
		&self.dirs * distance + &self.points
	}
}
