use vec::{Colour, abs};
use prelude::Material;
use types::Intersect;

#[derive(Copy, Clone)]
pub struct NormalColour {}

impl NormalColour {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for NormalColour {
    fn base_colour(&self, isect: &Intersect) -> Colour {
        abs(Colour::new([
			isect.normal.x as f32,
			isect.normal.y as f32,
			isect.normal.z as f32
		]))
    }
}
