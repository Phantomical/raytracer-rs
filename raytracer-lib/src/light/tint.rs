use lib::*;
use lib::light::Light;
use serde::Serialize;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Tint<T: Light> {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    tint: Colour,
    light: T,
}

type_serialization_decl!("tint");

impl<T: Light> Tint<T> {
    pub fn new(light: T, tint: Colour) -> Self {
        Self {
            tint,
            light,
            tag: (),
        }
    }
}

impl<T: Light> Light for Tint<T>
where
    T: Serialize,
{
    fn illumination(&self, isect: &Intersection) -> Colour {
        let illum = self.light.illumination(isect);

        illum.x * self.tint
    }

    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        self.light.shadow_rays(isect)
    }
}
