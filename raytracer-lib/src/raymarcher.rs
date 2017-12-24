
use ray::*;
use intersection::*;
use material::Material;
use object::Raymarchable;

use std::sync::Arc;
use std;

pub struct RaymarchOptions {
	// The maximum distance out to which a ray will be traced
	pub max_distance : f64,
	// The distance that the ray must be within to be considered
	// to have hit the surface
	pub intersect_distance : f64
}

pub fn raymarch(
	ray : Ray, 
	objects : &[(Arc<Raymarchable>, Arc<Material>)], 
	options : &RaymarchOptions) -> Option<Intersection>
{
	let mut dist : f64 = 0.0;

	while dist < options.max_distance {
		let mut step = std::f64::MAX;
		let point = ray.point_at(dist);

		for obj in objects {
			let objdist = obj.0.distance(point);

			if objdist < options.intersect_distance {
				return Some(Intersection::new(
					point,
					ray,
					Arc::clone(&obj.1),
					Arc::clone(&obj.0)));
			}

			if objdist < step {
				step = objdist;
			}
		}

		dist += step;
	}

	return None;
}
