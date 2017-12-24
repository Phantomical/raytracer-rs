
use vec::*;
use ray::*;
use std::sync::Arc;

use material::Material;
use object::Raymarchable;

pub struct Intersection
{
	pub point : Vec3d,
	pub ray : Ray,
	pub normal : Vec3d,
	
	pub material : Arc<Material>,
	pub object   : Arc<Raymarchable>
}

impl Intersection {
	pub fn new(
		point : Vec3d,
		ray   : Ray,
		material : Arc<Material>,
		object   : Arc<Raymarchable>) -> Intersection {
		return Intersection {
			normal   : object.normal_at(point, ray.direction),
			material : material,
			object   : object,
			point : point,
			ray   : ray,
		};
	}

	/* Expose Material Functions */
	pub fn colour(&self) -> Colour {
		return self.material.base_colour(self.point);
	}
	pub fn roughness(&self) -> f32 {
		return self.material.roughness(self.point);
	}
	pub fn reflectivity(&self) -> f32 {
		return self.material.reflectivity(self.point);
	}	
}
