
use lib::*;
use lib::light::Light;

use rand::{Rng, thread_rng, ThreadRng};
use rand::distributions::{IndependentSample, Range};

use std::ops::{Generator, GeneratorState};
use std::f64::consts::PI;

struct GenIter<G: Generator<Return = ()>>(G);

impl<G: Generator<Return = ()>> Iterator for GenIter<G> {
	type Item = G::Yield;

	fn next(&mut self) -> Option<Self::Item> {
		match self.0.resume() {
			GeneratorState::Yielded(y)   => Some(y),
			GeneratorState::Complete(()) => None
		}
	}
}

fn generator_to_iterator<G>(g: G) -> GenIter<G>
	where G: Generator<Return = ()>
{
    GenIter(g)
}

const DIRECTIONAL_DISTANCE : f64 = 1.0e10;

#[derive(Clone, Copy, Deserialize)]
pub struct FuzzyDirectionalLight {
	/// The main direction that the light is pointing in
	pub direction : Vec3d,
	/// Angular radius of the cone (radians)
	pub fuzziness : f64
}

fn orthagonal(a : Vec3d) -> Vec3d {
	assert!(a != Vec3d::zero());

	return Vec3d::new(1.0, 1.0, -(a.x + a.y) / a.z).normalize();
}

impl FuzzyDirectionalLight {
	pub fn new(dir : Vec3d, fuzziness : f64) -> FuzzyDirectionalLight {
		return FuzzyDirectionalLight {
			direction: dir.normalize(),
			fuzziness: fuzziness
		};
	}

	fn rand_vec<R: Rng>(&self, mut rng : &mut Rng) -> Vec3d {
		let u = Vec3d::cross(self.direction, orthagonal(self.direction));
		let v = Vec3d::cross(self.direction, u);

		let theta = Range::new(self.fuzziness.cos(), 1.0)
			.ind_sample(&mut rng).acos();
		let phi = Range::new(0.0f64, 2.0 * PI).ind_sample(&mut rng);

		return theta.sin() * (phi.cos() * u + phi.sin() * v) 
			+ theta.cos() * (-self.direction);
	}
}

impl Light for FuzzyDirectionalLight {
	fn illumination(&self, isect : &Intersection) -> Colour {
		let mult = (dot(isect.normal, -self.direction) as f32).abs();
		return Colour::new(mult, mult, mult);
	}

	fn shadow_rays(&self, isect : &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
		return Box::new(generator_to_iterator( { 
			let me = self.clone();
			let point = isect.point;
			let normal= isect.normal;
			move || {
				for _ in 0..20 {
					let mut rng = thread_rng();
					let vec = me.rand_vec::<ThreadRng>(&mut rng);
					yield (
						Ray::new(point + normal * 0.0001, vec), 
						DIRECTIONAL_DISTANCE
					);
				}
			}
		}));
	}
}
