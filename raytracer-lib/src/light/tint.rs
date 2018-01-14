use lib::*;
use lib::light::Light;

use std::sync::Arc;

pub struct Tint {
    tint: Colour,
    light: Arc<Light>,
}

impl Tint {
    pub fn new(light: Arc<Light>, tint: Colour) -> Self {
        Self { tint, light }
    }
}

impl Light for Tint {
    fn illumination(&self, isect: &Intersection) -> Colour {
        let illum = self.light.illumination(isect);

        Colour::new(
            illum.x * self.tint.x,
            illum.y * self.tint.y,
            illum.z * self.tint.z,
        )
    }

    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        self.light.shadow_rays(isect)
    }
}
