
use lib::light::Light;
use std::sync::Arc;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct LightData {
	#[serde(deserialize_with = "::serialize::light::deserialize")]
	pub light: Arc<Light>
}

impl LightData {
	pub fn new(light: Arc<Light>) -> Self {
		Self { light }
	}
}
