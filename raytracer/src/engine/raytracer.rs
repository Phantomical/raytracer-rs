
use std::vec::Vec;

use ndarray::{Array};

use types::{Intersect, Ray, RaytraceOptions, Scene, ObjectData};
use vec::{Colour, max};

pub struct TraceResult {
	index: (usize, usize),
	isect: Option<Intersect>
}

fn raytrace(
	objects: &[ObjectData],
	opts: &RaytraceOptions,
	rays: Ray) -> Vec<(f64, usize)>
{
	let mut distance = array![opts.min_distance].into_dyn();

	let indices: Vec<usize> = (0..rays.dirs.elem_size()).collect();
	let mut results = vec![];

	while rays.dirs.elem_size() > 0 {
		let points = rays.points_at(&distance);
		let mut step = array![opts.max_distance].into_dyn();

		for obj in objects {
			let mut dists = obj.object.distance(&points);
			azip!(mut dists, step, *dists = step.min(dists));
			step = dists;
		}

		distance = step + distance;

		let result: Vec<(f64, usize)> = distance.iter().zip(indices.iter())
			.filter(|&(x, _)| *x < opts.isect_distance || *x > opts.max_distance)
			.map(|(&x, &y)| (x, y))
			.collect();

		distance = Array::from_iter(distance.into_iter()
			.filter(|&x| *x < opts.isect_distance && *x > opts.max_distance)
			.map(|x| *x)).into_dyn();

		for val in result.into_iter() {
			results.push(val);
		}
	}

	return results;
}

fn intersect(
	objects: &[ObjectData],
	opts: &RaytraceOptions,
	rays: Ray) -> Vec<(Option<Intersect>, usize)>
{
	let dists = raytrace(objects, opts, rays.clone());
	let mut results = vec![];

	for (dist, i) in dists.into_iter() {
		if dist >= opts.max_distance {
			results.push((None, i));
		}
		else {
			let mut intersected = None;
			let point = rays.points.at(*&[i]) + dist * rays.dirs.at(*&[i]);

			for obj in objects.iter() {
				if obj.object.distance_singular(point) < opts.isect_distance {
					intersected = Some(Intersect{
						object: obj.clone(),
						normal: obj.object.normal_singular(point),
						position: point
					});
					break;
				} 
			}

			results.push((intersected, i));
		}
	}

	return results;
}

fn illuminate<'a, I>(
	isects: I,
	scene: &Scene,
	opts: &RaytraceOptions) -> Vec<(Colour, usize)>
where I: Iterator<Item = &'a Intersect>
{
	isects.enumerate().map(|(i, ref isect)| {
		let mut illum = Colour::zero();

		for ref light in &scene.lights {
			let new_illum = light.illumination(isect);

			let mut isects = intersect(
				&scene.objects,
				opts,
				light.shadow_rays(isect));

			let rays = isects.len();
			let visible: usize = isects.iter()
				.map(|&(ref x, _)| match x { &None => 0, &Some(_) => 1 })
				.sum();

			let mult = if rays == 0 && visible == 0 {
				(visible as f32) / (rays as f32)
			} else {
				1.0
			};

			illum = max(illum, new_illum * mult);
		}

		(illum, i)
	}).collect::<Vec<(Colour, usize)>>()
}

fn base_colour<'a, I>(isects: I) -> impl Iterator<Item = Colour>
where I: Iterator<Item = &'a Intersect>
{
	isects.map(|isect| isect.object.material.base_colour(isect))
}

fn shade(
	scene: &Scene,
	opts: &RaytraceOptions,
	background: Colour,
	rays: Ray) -> Vec<Colour>
{
	let isects = intersect(
		&scene.objects,
		opts,
		rays
	);
	
	let filt: Vec<(Intersect, usize)> = isects.iter()
		.filter(|&&(ref x, _)| x.is_some())
		.map(|&(ref x, y)| {
			match x {
				&Some(ref x) => (x.clone(), y),
				&None => unreachable!()
			}
		})
		.collect();

	let colours: Vec<Colour> = base_colour(
		filt.iter().map(|&(ref x, _)| x)).collect();
	let illum: Vec<(Colour, usize)> = illuminate(
		filt.iter().map(|&(ref x, _)| x), 
		&scene, 
		opts);

	let iter = {
		let it1 = filt.iter().map(|&(_, i)| i);
		let it2 = colours.iter();
		let it3 = illum.iter().map(|&(ref x, _)| x);

		it1.zip(it2.zip(it3).map(|(x, y)| *x * *y))
	};

	let mut results = vec![];

	for (i, colour) in iter {
		results.push((colour, i));
	}

	for i in isects.into_iter()
		.filter(|&(ref x, _)| x.is_none())
		.map(|(_, i)| i) 
	{
		results.push((background, i));
	}

	results.sort_by(|a, b| a.1.cmp(&b.1));

	return results.into_iter()
		.map(|(x, _)| x)
		.collect::<Vec<Colour>>();
}
