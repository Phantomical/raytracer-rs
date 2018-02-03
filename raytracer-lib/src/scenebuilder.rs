
use newscene::*;
use cacheable::*;

use lib::light::Light;
use lib::object::Raymarchable;
use lib::material::Material;
use lib::Colour;

use std::vec::Vec;
use std::rc::Rc;
use std::sync::Arc;

pub struct SceneBuilder {
	objects: Vec<CachedObjectData>,
	lights: Vec<Arc<Cacheable<Rc<Light>>>>,
	background: Colour,
}

impl SceneBuilder {
	pub fn new() -> Self{
		Self{ 
			objects: Vec::new(),
			lights:  Vec::new(),
			background: Colour::zero()
		}
	}

	pub fn add_object<O, M>(mut self, obj: O, mat: M) -> Self
		where O: Cacheable<Rc<Raymarchable>> + Sized + 'static,
		      M: Cacheable<Rc<Material>> + Sized + 'static
	{
		self.objects.push(CachedObjectData {
			object: Arc::new(obj),
			material: Arc::new(mat),
			bounds: None
		});

		self
	}
	pub fn add_light<L>(mut self, light: L) -> Self
		where L: Cacheable<Rc<Light>> + 'static
	{
		self.lights.push(Arc::new(light));

		self
	}
	pub fn background(mut self, colour: Colour) -> Self {
		self.background = colour;
		self
	}

	pub fn unwrap(self) -> CachedScene {
		CachedScene {
			background: self.background,
			lights: self.lights,
			objects: self.objects,
		}
	}
}