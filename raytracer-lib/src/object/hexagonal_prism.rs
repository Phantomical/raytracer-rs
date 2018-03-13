use lib::*;
use lib::object::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct HexagonalPrism {
    radius: f64,
    height: f64,
}

impl HexagonalPrism {
    pub fn new(height: f64, radius: f64) -> Self {
        return Self { radius, height };
    }
}

impl Raymarchable for HexagonalPrism {
    fn distance(&self, p: Vec3d) -> f64 {
        let q = abs(p);
        return (q.z - self.height).max((q.x * 0.866025 + q.y * 0.5).max(q.y) - self.radius);
    }
}
