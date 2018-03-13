use lib::*;
use std::sync::Arc;
use light::Light;

use std::vec::Vec;

pub struct Scene {
    pub lights: Vec<Arc<Light>>,
    pub objects: Vec<ObjectData>,
    pub background: Colour,
}

impl Scene {
    pub fn new(bg: Colour) -> Scene {
        return Scene {
            background: bg,
            objects: Vec::new(),
            lights: Vec::new(),
        };
    }
}
