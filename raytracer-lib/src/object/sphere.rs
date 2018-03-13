use vec::*;
use ray::*;
use object::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Sphere {
    centre: Vec3d,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Vec3d, radius: f64) -> Sphere {
        return Sphere {
            centre: centre,
            radius: radius,
        };
    }
}

impl Analytical for Sphere {
    fn intersect(&self, _: &Ray) -> Option<Vec3d> {
        unimplemented!();
    }
}

impl Raymarchable for Sphere {
    fn normal_at(&self, point: Vec3d, _dir: Vec3d) -> Vec3d {
        return (point - self.centre).normalize();
    }
    fn distance(&self, point: Vec3d) -> f64 {
        return distance(point, self.centre) - self.radius;
    }
}
