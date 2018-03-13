use lib::*;
use lib::object::IFS;
use lib::material::Material;

use std::iter::*;

use gradient::Gradient;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct OriginTrap<T> {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    gradient: Gradient<f32, Colour>,
    object: T,
}

type_serialization_decl!("origin_trap");

impl<T: IFS> OriginTrap<T> {
    pub fn new(gradient: Gradient<f32, Colour>, object: T) -> Self {
        Self {
            gradient,
            object,
            tag: (),
        }
    }
}

impl<T: IFS> Material for OriginTrap<T>
where
    T: Serialize + Sync + Send,
{
    fn base_colour(&self, isect: &Intersection) -> Colour {
        let min = self.object
            .points(isect.point)
            .map(|v| v.length())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        return self.gradient.value_at(min as f32);
    }
}
