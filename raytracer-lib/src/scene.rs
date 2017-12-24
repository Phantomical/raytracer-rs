
use vec::*;
use camera::*;
use object::Raymarchable;
use material::Material;
use raymarcher::*;

use std::sync::Arc;
use std::vec::Vec;

pub struct Scene {
	pub objects : Vec<(Arc<Raymarchable>, Arc<Material>)>,
	pub camera  : Camera,
	pub options : RaymarchOptions
}

impl Scene {
	pub fn new(cam : Camera, opt : RaymarchOptions) -> Scene {
		return Scene {
			camera  : cam,
			options : opt,
			objects : Vec::new()
		};
	}


}
