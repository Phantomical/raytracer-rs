use lib::*;
use lib::object::IFS;
use lib::material::Material;

use std::iter::*;

use cacheable::Cacheable;
use gradient::Gradient;

pub struct OriginTrap<T> {
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

impl<R: IFS, T: Cacheable<R>> Cacheable<OriginTrap<R>> for OriginTrap<T> {
	fn cached(&self) -> OriginTrap<R> {
		OriginTrap::new(self.gradient.clone(), self.object.cached())
	}
}
