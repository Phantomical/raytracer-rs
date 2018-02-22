
use prelude::*;

use std::vec::Vec;
use std::sync::Arc;

use types::ObjectData;

pub struct Scene {
	pub objects:   Vec<ObjectData>,
	pub lights:    Vec<Arc<Light>>
}
