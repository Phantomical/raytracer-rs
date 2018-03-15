#![allow(unused_variables)]

use lib::*;
use lib::prelude::*;
use std::sync::Arc;

use rand::{random, Closed01};

fn light_illumination(
    scene: &Scene,
    isect: &Intersection,
    light: &Arc<Light>,
    opts: &RaymarchOptions,
) -> Colour {
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
            None => 1,
        };
    }

    if count == 0 && visible == 0 {
        count = 1;
        visible = 1;
    }

    return light.illumination(isect) * ((count as f32) / (visible as f32));
}

pub fn illuminate(
	scene: &Scene, 
	isect: &Intersection, 
	opts: &RaymarchOptions) -> Colour 
{
    let base = isect.object.material.base_colour(isect);
    let mut illum = Colour::zero();

    for ref light in scene.lights.iter() {
        let light_illum = light_illumination(scene, isect, &light.light, opts);

        illum = max(light_illum, illum);
    }

    return base * clamp(illum, 0.0, 1.0);
}

fn reflected_colour_internal(
	scene: &Scene, 
	isect: &Intersection, 
	opts: &RaymarchOptions,
	depth: u32) -> Colour 
{
	let ray = Ray::new(isect.point, reflect(isect.ray.direction, isect.normal));
	let opts = RaymarchOptions {
		min_distance: opts.intersect_distance * 4.0,
		..*opts
	};

	return trace_ray_internal(scene, ray, &opts, depth);
}

fn isect_colour_internal(
	scene: &Scene,
	isect: &Intersection,
	opts: &RaymarchOptions,
	depth: u32) -> Colour
{
	let surface_colour = illuminate(scene, isect, opts);
	
	let reflectivity = isect.object.material.reflectivity(isect);

	let reflect_colour = if reflectivity > MIN_REFLECTIVITY {
		reflected_colour_internal(scene, isect, opts, depth)
	}
	else {
		scene.background
	};

	return mix(reflect_colour, surface_colour, reflectivity);
}

const MAX_RECURSION_DEPTH: u32 = 100;
const MIN_REFLECTIVITY: f32 = 1e-3;

fn trace_ray_internal(
	scene: &Scene,
	ray: Ray,
	opts: &RaymarchOptions,
	depth: u32) -> Colour
{
	if depth > MAX_RECURSION_DEPTH {
		return scene.background;
	}

	match raymarch(ray, &scene.objects, opts) {
		Some(isect) => isect_colour_internal(scene, &isect, opts, depth + 1),
		None        => scene.background
	}
}

pub fn trace_ray(
	scene: &Scene,
	ray: Ray,
	opts: &RaymarchOptions
) -> Colour
{
	trace_ray_internal(scene, ray, opts, 0)
}

pub fn trace_pixel(desc: &ImageDesc, pixel: Vec2u) -> Colour {
    fn randval() -> f64 {
        let Closed01(val) = random::<Closed01<f64>>();
        return val;
    }
    
	let result: Colour = (0..desc.size.samples)
		.map(|_| {
			let point = vec2(
				((pixel.x as f64) / (desc.size.width  as f64)) * 2.0 - 1.0,
				((pixel.y as f64) / (desc.size.height as f64)) * 2.0 - 1.0);

			let jitter = vec2(
				randval() / (desc.size.width as f64),
				randval() / (desc.size.height as f64));

			let pt = point + jitter;
			trace_ray(&desc.scene, desc.camera.screen_ray(pt), &desc.opts)
		}).fold(Colour::zero(), |sum, val| sum + val);

	return result / (desc.size.samples as f32);
}
