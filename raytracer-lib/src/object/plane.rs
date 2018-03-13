use vec::*;
use object::*;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Plane {
	#[serde(with = "tag")]
	#[serde(rename = "type")]
	#[serde(skip_deserializing)]
	tag: (),


    /// The normal vector of the plane.
    /// All vectors laying on the plane
    /// are perpendicular to this vector.
    normal: Vec3d,
    // A point on the plane
    point: Vec3d,
}

type_serialization_decl!("plane");

impl Plane {
    pub fn new(normal: Vec3d, point: Vec3d) -> Plane {
        return Plane {
            normal: normal.normalize(),
            point: point,
			tag: (),
        };
    }
}

impl Analytical for Plane {}

impl Raymarchable for Plane {
    fn distance(&self, point: Vec3d) -> f64 {
        return dot(self.normal, point - self.point);
    }

    fn normal_at(&self, _point: Vec3d, _dir: Vec3d) -> Vec3d {
        return self.normal;
    }
}
