
use lib::*;
use lib::object::Raymarchable;

#[derive(Clone)]
pub struct BoundSphere<T: Raymarchable + Clone> {
	radius: f64,
	object: T
}

impl<T> BoundSphere<T>
	where T: Raymarchable + Clone
{
	pub fn new(radius: f64, object: T) -> Self {
		Self { radius, object }
	}
}

impl<T> Raymarchable for BoundSphere<T>
	where T: Raymarchable + Clone 
{
	fn distance(&self, point: Vec3d) -> f64 {
		let dist = length(point);

		if dist < self.radius + 1.0 {
			return self.object.distance(point);
		}
		return dist;
	}
}
