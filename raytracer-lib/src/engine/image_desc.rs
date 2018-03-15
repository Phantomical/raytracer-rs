use lib::{Camera, ImageSize, RaymarchOptions, Scene};
use std::sync::Arc;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct ImageDesc {
    pub scene: Arc<Scene>,
    pub camera: Camera,

    pub size: ImageSize,
    pub opts: RaymarchOptions,
}
