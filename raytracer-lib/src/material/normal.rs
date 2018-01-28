use lib::*;
use lib::material::Material;

pub struct NormalColour {}

impl NormalColour {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for NormalColour {
    fn base_colour(&self, isect: &Intersection) -> Colour {
        let normal = isect.normal;
        vec3(
            normal.x.abs() as f32,
            normal.y.abs() as f32,
            normal.z.abs() as f32,
        )
    }
}
