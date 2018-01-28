use lib::*;
use lib::light::Light;

use std::iter::once;

pub struct PointLight {
    power: f32,
    position: Vec3d,
}

impl PointLight {
    pub fn new(position: Vec3d, power: f32) -> Self {
        Self { position, power }
    }
}

impl Light for PointLight {
    fn illumination(&self, isect: &Intersection) -> Colour {
        let val = self.power / (distance2(self.position, isect.point) as f32);

        Colour::new([val, val, val])
    }

    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        Box::new(once((
            Ray::new(self.position, (self.position - isect.point).normalize()),
            distance(self.position, isect.point),
        )))
    }
}
