use lib::*;

#[derive(Clone)]
pub struct Intersection {
    pub point: Vec3d,
    pub ray: Ray,
    pub normal: Vec3d,
    pub object: ObjectData,
}
