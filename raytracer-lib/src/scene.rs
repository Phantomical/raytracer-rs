
use vec::*;
use camera::*;
use raymarcher::*;
use intersection::*;
use material::Material;
use object::Raymarchable;

use std::sync::Arc;
use std::vec::Vec;

pub struct Scene {
	pub objects : Vec<(Arc<Raymarchable>, Arc<Material>)>,
	pub camera  : Camera,
	pub options : RaymarchOptions,
	pub background : Colour
}

impl Scene {
	pub fn new(
		cam : Camera, 
		opt : RaymarchOptions, 
		bg : Colour) -> Scene 
	{
		return Scene {
			camera  : cam,
			options : opt,
			objects : Vec::new(),
			background : bg
		};
	}

	fn isect_colour(&self, isect : &Intersection) -> Colour {
		return isect.material.base_colour(isect.point);
	}

	pub fn trace_point(&self, point : Vec2d) -> Colour {
		let isect = raymarch(
			self.camera.screen_ray(point),
			&self.objects,
			&self.options);
		
		return match isect {
			None => self.background,
			Some(isect) => self.isect_colour(&isect)				
		}
	}
}
