
use lib::object::{Raymarchable, Analytical};
use lib::material::Material;
use lib::light::Light;
use std::vec::Vec;
use std::sync::Arc;

pub struct ObjectData {
	pub object   : Arc<Raymarchable>,
	pub bound    : Option<Arc<Analytical>>,
	pub material : Arc<Material>
}

#[derive(Clone, Default)]
pub struct SceneData {
	pub lights : Vec<Arc<Light>>
}

impl SceneData {
	pub fn new() -> Self {
		return Self{ ..Default::default() };
	}
	pub fn add_light(&mut self, light : &Arc<Light>) {
		self.lights.push(Arc::clone(light));
	}
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
