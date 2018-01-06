
use lib::*;
use std::sync::Arc;
use lib::object::Raymarchable;
use material::Material;
use light::Light;

use std::vec::Vec;

pub struct Scene {
	pub data    : SceneData,
	pub objects : Vec<ObjectData>,
	pub camera  : Camera,
	pub options : RaymarchOptions,
	pub background : Colour
}

fn vec_max(a : Colour, b : Colour) -> Colour {
	return Colour::new(
		a.x.max(b.x),
		a.y.max(b.y),
		a.z.max(b.z));
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
			background : bg,
			objects : Vec::new(),
			data    : SceneData::default()
		};
	}

	fn isect_colour(&self, isect : &Intersection) -> Colour {
		let base_colour = isect.object.material.base_colour(isect.point);
		let mut illum = Colour::zero();

		for ref light in self.data.lights.iter() {
			let (ray, maxdist) = light.shadow_ray(isect);

			// Conserve any additional options
			let mut opts = self.options.clone();
			opts.min_distance = opts.intersect_distance * 5.0;
			opts.max_distance = maxdist;

			let test = raymarch(ray, &self.objects,	&opts);

			illum = vec_max(match test {
				None    => light.illumination(isect),
				Some(_) => Colour::zero()
			}, illum);			
		};

		return Colour::new(
			base_colour.x * illum.x.min(1.0).max(0.0),
			base_colour.y * illum.y.min(1.0).max(0.0),
			base_colour.z * illum.z.min(1.0).max(0.0));
	}

	pub fn trace_ray(&self, ray : Ray) -> Colour {
		let isect = raymarch(
			ray,
			&self.objects,
			&self.options);
		
		return match isect {
			None        => self.background,
			Some(isect) => self.isect_colour(&isect)				
		}
	}

	pub fn trace_point(&self, point : Vec2d) -> Colour {
		return self.trace_ray(self.camera.screen_ray(point));
	}

	pub fn add_object(&mut self, obj : (Arc<Raymarchable>, Arc<Material>)) {
		self.objects.push(ObjectData{ 
			object:   obj.0, 
			material: obj.1,
			bound:    None,
		});
	}
	pub fn add_light(&mut self, light : Arc<Light>) {
		self.data.lights.push(light);
	}
}
