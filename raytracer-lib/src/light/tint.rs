use lib::*;
use lib::light::Light;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Tint<T: Light> {
    tint: Colour,
    light: T,
}

impl<T: Light> Tint<T> {
    pub fn new(light: T, tint: Colour) -> Self {
        Self { tint, light }
    }
}

impl<T: Light> Light for Tint<T> {
    fn illumination(&self, isect: &Intersection) -> Colour {
        let illum = self.light.illumination(isect);

        illum.x * self.tint
    }

    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        self.light.shadow_rays(isect)
    }
}
