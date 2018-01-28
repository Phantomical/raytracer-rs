use lib::*;
use lib::object::IFS;
use lib::material::Material;

use std::iter::*;

use gradient::Gradient;

pub struct OriginTrap<T: IFS> {
    gradient: Gradient<f32, Colour>,
    object: T,
}

impl<T: IFS> OriginTrap<T> {
    pub fn new(gradient: Gradient<f32, Colour>, object: T) -> Self {
        Self { gradient, object }
    }
}

impl<T: IFS> Material for OriginTrap<T> {
    fn base_colour(&self, isect: &Intersection) -> Colour {
        let min = self.object
            .points(isect.point)
            .map(|v| v.length())
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        //println!("{}", min);

        return self.gradient.value_at(min as f32);
    }
}
