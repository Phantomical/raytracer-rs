use lib::*;
use lib::object::Raymarchable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BoundSphere<T: Raymarchable> {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    radius: f64,
    object: T,
}

impl<T> BoundSphere<T>
where
    T: Raymarchable,
{
    pub fn new(radius: f64, object: T) -> Self {
        Self {
            radius,
            object,
            tag: (),
        }
    }
}

type_serialization_decl!("boundsphere");

impl<'de, T> Raymarchable for BoundSphere<T>
where
    T: Raymarchable + Serialize + Deserialize<'de>,
{
    fn distance(&self, point: Vec3d) -> f64 {
        let dist = length(point);

        if dist < self.radius + 1.0 {
            return self.object.distance(point);
        }
        return dist;
    }
}
