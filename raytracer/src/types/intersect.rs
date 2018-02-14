
use vec::Vec3d;
use prelude::*;

use std::sync::Arc;

pub struct Intersect {
	pub position: Vec3d,
	pub normal: Vec3d,
	pub object: Arc<Object>,
	pub material: Arc<Material>
}

impl Intersect {

}
