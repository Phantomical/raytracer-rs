
use lib::object::{Raymarchable, Analytical};
use lib::material::Material;
use std::sync::Arc;

pub struct ObjectData {
	pub object   : Arc<Raymarchable>,
	pub bound    : Option<Arc<Analytical>>,
	pub material : Arc<Material>
}

impl Clone for ObjectData {
	fn clone(&self) -> Self {
		return Self {
			object:   Arc::clone(&self.object),
			material: Arc::clone(&self.material),
			bound:    match self.bound {
				Some(ref v) => Some(Arc::clone(v)),
				None        => None
			}
		};
	}
}
