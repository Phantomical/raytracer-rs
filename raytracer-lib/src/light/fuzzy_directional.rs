use lib::*;
use lib::light::Light;

use rand::{thread_rng, Rng, ThreadRng};
use rand::distributions::{IndependentSample, Range};

use std::ops::{Generator, GeneratorState};
use std::f64::consts::PI;

struct GenIter<G: Generator<Return = ()>>(G);

impl<G: Generator<Return = ()>> Iterator for GenIter<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            GeneratorState::Yielded(y) => Some(y),
            GeneratorState::Complete(()) => None,
        }
    }
}

fn generator_to_iterator<G>(g: G) -> GenIter<G>
where
    G: Generator<Return = ()>,
{
    GenIter(g)
}

const DIRECTIONAL_DISTANCE: f64 = 1.0e10;

#[derive(Clone, Copy)]
pub struct FuzzyDirectionalLight {
    /// The main direction that the light is pointing in
    pub direction: Vec3d,
    /// Angular radius of the cone (radians)
    pub fuzziness: f64,
    pub rays: usize,
}

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
        return Box::new(generator_to_iterator({
            let me = self.clone();
            let point = isect.point;
            let normal = isect.normal;
            move || {
                for _ in 0..me.rays {
                    let mut rng = thread_rng();
                    let vec = me.rand_vec::<ThreadRng>(&mut rng);
                    yield (Ray::new(point + normal * 0.001, vec), DIRECTIONAL_DISTANCE);
                }
            }
        }));
    }
}
