use lib::object::{Analytical, Raymarchable};
use lib::material::Material;
use std::sync::Arc;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct ObjectData {
	#[serde(deserialize_with = "::serialize::object::deserialize")]
    pub object: Arc<Raymarchable>,
	#[serde(skip)]
    pub bound: Option<Arc<Analytical>>,
	#[serde(deserialize_with = "::serialize::material::deserialize")]
    pub material: Arc<Material>,
}


