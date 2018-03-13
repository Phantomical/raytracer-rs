use lib::*;
use lib::light::Light;

use rand::{thread_rng, Rng, ThreadRng};
use rand::distributions::{IndependentSample, Range};

use std::vec::Vec;
use std::f64::consts::PI;

const DIRECTIONAL_DISTANCE: f64 = 1.0e10;

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct FuzzyDirectionalLight {
    #[serde(with = "tag")]
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    tag: (),

    /// The main direction that the light is pointing in
    pub direction: Vec3d,
    /// Angular radius of the cone (radians)
    pub fuzziness: f64,
    pub rays: usize,
}

type_serialization_decl!("fuzzy_directional_light");

fn orthagonal(a: Vec3d) -> Vec3d {
    //TODO: Implement this
    //assert!(a != Vec3d::zero());

    return vec3(1.0, 1.0, -(a.x + a.y) / a.z).normalize();
}

impl FuzzyDirectionalLight {
    pub fn new(dir: Vec3d, fuzziness: f64, rays: usize) -> FuzzyDirectionalLight {
        return FuzzyDirectionalLight {
            direction: dir.normalize(),
            fuzziness,
            rays,
			tag: ()
        };
    }

    fn rand_vec<R: Rng>(&self, mut rng: &mut Rng) -> Vec3d {
        let u = cross(self.direction, orthagonal(self.direction));
        let v = cross(self.direction, u);

        let theta = Range::new(self.fuzziness.cos(), 1.0)
            .ind_sample(&mut rng)
            .acos();
        let phi = Range::new(0.0f64, 2.0 * PI).ind_sample(&mut rng);

        return theta.sin() * (phi.cos() * u + phi.sin() * v) + theta.cos() * (-self.direction);
    }
}

impl Light for FuzzyDirectionalLight {
    fn illumination(&self, isect: &Intersection) -> Colour {
        Colour::new([(dot(isect.normal, -self.direction) as f32).abs(); 3])
    }

    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        let mut points = Vec::new();
        for _ in 0..self.rays {
            let mut rng = thread_rng();
            let vec = self.rand_vec::<ThreadRng>(&mut rng);
            points.push((
                Ray::new(isect.point + isect.normal * 0.001, vec),
                DIRECTIONAL_DISTANCE,
            ));
        }

        return Box::new(points.into_iter());
    }
}
