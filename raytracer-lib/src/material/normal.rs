use lib::*;
use lib::material::Material;
use lib::object::Raymarchable;

use std::sync::Arc;

pub struct NormalColour {
    obj: Arc<Raymarchable>,
}

impl NormalColour {
    pub fn new(obj: Arc<Raymarchable>) -> Self {
        Self { obj }
    }
}

impl Material for NormalColour {
    fn base_colour(&self, point: Vec3d) -> Colour {
        let normal = self.obj.normal_at(point, Vec3d::zero());
        Colour::new(normal.x as f32, normal.y as f32, normal.z as f32)
    }
}
