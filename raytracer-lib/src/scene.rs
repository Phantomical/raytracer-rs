
use vec::*;
use object::Raymarchable;
use material::Material;

use std::sync::Arc;
use std::vec::Vec;

pub struct Scene {
	pub objects : Vec<(Arc<Raymarchable>, Arc<Material>)>
}
