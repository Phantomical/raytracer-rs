use lib::*;
use light::*;
use std::iter::once;

const DIRECTIONAL_DISTANCE: f64 = 1.0e10;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct DirectionalLight {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    direction: Vec3d,
}

type_serialization_decl!("directional_light");

impl Light for DirectionalLight {
    fn illumination(&self, isect: &Intersection) -> Colour {
        Colour::new([(dot(isect.normal, -self.direction) as f32).abs(); 3])
    }

    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        return Box::new(once((
            Ray::new(isect.point + isect.normal * 0.001, -self.direction),
            DIRECTIONAL_DISTANCE,
        )));
    }
}

impl DirectionalLight {
    pub fn new(direction: Vec3d) -> DirectionalLight {
        return DirectionalLight {
            direction: direction,
			tag: ()
        };
    }
}
