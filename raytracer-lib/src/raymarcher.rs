use lib::*;
use std;

#[derive(Clone, Copy)]
pub struct RaymarchOptions {
    /// The maximum distance out to which a ray will be traced
    pub max_distance: f64,
    /// The minumum distance that a ray must traverse before intersecting
    pub min_distance: f64,
    /// The distance that the ray must be within to be considered
    /// to have hit the surface
    pub intersect_distance: f64,
}

impl Default for RaymarchOptions {
    fn default() -> Self {
        return Self {
            max_distance: 1.0e6,
            min_distance: 1.0e-6,
            intersect_distance: 1.0e-8,
        };
    }
}

pub fn raymarch(
    ray: Ray,
    objects: &Vec<ObjectData>,
    options: &RaymarchOptions,
) -> Option<Intersection> {
    let mut dist: f64 = options.min_distance;

    while dist < options.max_distance {
        let mut step = std::f64::MAX;
        let point = ray.point_at(dist);

        for obj in objects {
            let ref raymarchable = obj.object;
            let objdist = raymarchable.distance(point);

            if objdist < options.intersect_distance {
                return Some(Intersection {
                    point: point,
                    normal: raymarchable.normal_at(point, ray.direction),
                    ray: ray,
                    object: obj.clone(),
                });
            }

            if objdist < step {
                step = objdist;
            }
        }

        dist += step;
    }

    return None;
}
