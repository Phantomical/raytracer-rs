use lib::*;
use std;

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
