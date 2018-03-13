use lib::*;
use lib::object::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BoxObj {
    pub bounds: Vec3d,
}

impl BoxObj {
    pub fn new(bounds: Vec3d) -> Self {
        Self { bounds }
    }
}

impl Raymarchable for BoxObj {
    fn distance(&self, point: Vec3d) -> f64 {
        let d = abs(point) - self.bounds;

        return d.x.max(d.y.max(d.z)).min(0.0) + length(max(d, Vec3d::zero()));
    }
}
