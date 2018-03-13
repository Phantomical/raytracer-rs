use lib::*;

use std::vec::Vec;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub lights: Vec<LightData>,
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
