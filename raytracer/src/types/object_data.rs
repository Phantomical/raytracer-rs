
use std::sync::Arc;

use prelude::*;

#[derive(Clone)]
pub struct ObjectData {
	pub object:   Arc<Object>,
	pub material: Arc<Material>
}