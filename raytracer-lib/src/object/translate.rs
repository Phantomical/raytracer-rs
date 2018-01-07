
use lib::*;
use lib::object::*;
use serialization::Deserialize;

#[derive(Deserialize)]
pub struct Translate<T: Raymarchable + Sized + Deserialize> {
	position : Vec3d,
	subobj   : T
}

impl<T: Raymarchable + Sized + Deserialize> Translate<T> {
	pub fn new(pos : Vec3d, obj : T) -> Self {
		return Self { 
			position: pos,
			subobj:   obj
		};
	}
}

impl<T> Raymarchable for Translate<T> 
	where T: Raymarchable + Sized + Deserialize
{
	fn normal_at(&self, point : Vec3d, dir : Vec3d) -> Vec3d {
		return self.subobj.normal_at(point - self.position, dir);
	}

	fn distance(&self, point : Vec3d) -> f64 {
		return self.subobj.distance(point - self.position);
	}
}

pub fn translate<T>(obj : T, pos : Vec3d) -> Translate<T> 
	where T: Raymarchable + Sized + Deserialize
{
	return Translate::new(pos, obj);
}
