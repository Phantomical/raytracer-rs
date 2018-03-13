
#![allow(unused_variables)]

use lib::*;
use lib::prelude::*;
use std::sync::Arc;

fn light_illumination(
	scene: &Scene,
	isect: &Intersection,
	light: &Arc<Light>,
	opts: &RaymarchOptions) -> Colour 
{
	let illum = light.illumination(isect);

	let mut count = 0;
	let mut visible = 0;

	for (ray, dist) in light.shadow_rays(isect) {
		let nopts = RaymarchOptions {
			max_distance: dist,
			min_distance: opts.intersect_distance * 2.0,
			..*opts
		};

		visible += 1;

		count += match raymarch(ray, &scene.objects, &nopts) {
			Some(_) => 0,
			None    => 1
		};
	}

	if count == 0 && visible == 0 {
		count = 1;
		visible = 1;
	}

	return illum * ((count as f32) / (visible as f32));
}

pub fn illuminate(
	scene: &Scene,
	isect: &Intersection, 
	opts: &RaymarchOptions) -> Colour 
{
	let base = isect.object.material.base_colour(isect);
	let mut illum = Colour::zero();

	for ref light in scene.lights.iter() {
		let light_illum = light_illumination(
			scene, isect, &light.light, opts);

		illum = max(light_illum, illum);
	}

	return base * illum;
}

pub fn trace_pixel(desc: &ImageDesc, pixel: Vec2u) -> Colour {
	unimplemented!();
}