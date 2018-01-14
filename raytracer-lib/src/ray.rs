use lib::*;

pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d,
}

impl Default for Ray {
    fn default() -> Self {
        return Self {
            origin: Vec3d::zero(),
            direction: -Vec3d::unit_z(),
        };
    }
}

impl Ray {
    pub fn new(orig: Vec3d, dir: Vec3d) -> Ray {
        return Ray {
            origin: orig,
            direction: dir.normalize(),
        };
    }

    pub fn point_at(&self, distance: f64) -> Vec3d {
        return self.origin + self.direction * distance;
    }
}
