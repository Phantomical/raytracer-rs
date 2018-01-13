
use lib::*;
use std::sync::Arc;
use lib::object::Raymarchable;
use material::Material;
use light::Light;

use std::vec::Vec;

pub struct Scene {
	pub lights  : Vec<Arc<Light>>,
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
		bg  : Colour) -> Scene 
	{
		return Scene {
			camera  : cam,
			options : opt,
			background : bg,
			objects : Vec::new(),
			lights  : Vec::new()
		};
	}

	fn isect_illumination(
		&self,
		isect : &Intersection, 
		light : &Arc<Light>) -> Colour
	{
		let mut count : usize = 0;
		let mut visible : usize = 0;

		for (ray, maxdist) in light.shadow_rays(isect) {
			let mut opts = self.options;
			opts.min_distance = opts.intersect_distance * 5.0;
			opts.max_distance = maxdist;

			let test = raymarch(ray, &self.objects, &opts);

			count += 1;
			visible += match test { Some(_) => 0, None => 1 };
		}

		if count == 0 { count = 1; visible = 1; }

		let mult = (visible as f32) / (count as f32);

		return match visible {
			0 => Colour::zero(),
			_ => light.illumination(isect) * mult
		};
	}

	fn isect_colour(&self, isect : &Intersection) -> Colour {
		let base_colour = isect.object.material.base_colour(isect.point);
		let mut illum = Colour::zero();

		for ref light in self.lights.iter() {
			illum = vec_max(
				self.isect_illumination(isect, light),
				illum);			
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

	pub fn add_object(&mut self, obj : Arc<Raymarchable>, mat : Arc<Material>) {
		self.objects.push(ObjectData{ 
			object:   obj, 
			material: mat,
			bound:    None,
		});
	}
	pub fn add_light(&mut self, light : Arc<Light>) {
		self.lights.push(light);
	}
}
