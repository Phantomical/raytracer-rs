
use vec::*;
use ray::Ray;

pub struct Intersection {
	// The point where the ray intersected the object
	point : Vec3d,
	// Normal vector of surface at intersection
	normal : Vec3d,

	// The ray that was intersected
	src_ray : Ray,
}
