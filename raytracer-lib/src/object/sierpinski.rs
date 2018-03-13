use lib::*;
use lib::object::Raymarchable;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Sierpinski {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    iterations: u32,
    scale: f64,
}

type_serialization_decl!("sierpinski");

impl Sierpinski {
    pub fn new(iterations: u32, scale: f64) -> Self {
        Self {
            iterations,
            scale,
            tag: (),
        }
    }
}

const A1: Vec3d = Vec3d {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const A2: Vec3d = Vec3d {
    x: -1.0,
    y: -1.0,
    z: 1.0,
};
const A3: Vec3d = Vec3d {
    x: 1.0,
    y: -1.0,
    z: -1.0,
};
const A4: Vec3d = Vec3d {
    x: -1.0,
    y: 1.0,
    z: -1.0,
};

impl Raymarchable for Sierpinski {
    fn distance(&self, point: Vec3d) -> f64 {
        let mut z = point;

        for _ in 0..self.iterations {
            let mut c = A1;
            let mut dist = (z - A1).length();

            let mut d = (z - A2).length();
            if d < dist {
                c = A2;
                dist = d;
            }

            d = (z - A3).length();
            if d < dist {
                c = A3;
                dist = d;
            }

            d = (z - A4).length();
            if d < dist {
                c = A4;
            }

            z = self.scale * z - c * (self.scale - 1.0);
        }

        return (z.length() - 2.0) / self.scale.powi(self.iterations as i32);
    }
}
