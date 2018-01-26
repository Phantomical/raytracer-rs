
use std::ops::*;
use std::vec::Vec;

pub use num::traits::One;

pub struct Gradient<T: Sized, C: Sized> {
	samples : Vec<(C, T)>
}

impl<T, C> Gradient<T, C> 
	where C: Add<Output = C> + Mul<T, Output = C> + Sized + Clone,
	      T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One
		   + PartialOrd + Sized + Clone + Div<Output = T>
{
	pub fn new(samples : &[(C, T)]) -> Self {
		let mut vec : Vec<(C, T)> = samples.iter()
				.map(|&(ref a, ref b)| (a.clone(), b.clone()))
				.collect();

		vec.sort_by(|ref a, ref b| (*a).1.partial_cmp(&(*b).1)
			.expect("Error: Unordered element position within gradient!"));

		Self { 
			samples: vec
		}
	}

	pub fn value_at(&self, value : T) -> C {
		if self.samples[0].1 > value {
			return self.samples[0].0.clone();
		}

		for i in 0..(self.samples.len() - 1) {
			if self.samples[i].1 < value {
				let t1 = self.samples[i].1.clone();
				let t2 = self.samples[i + 1].1.clone();

				let c1 = self.samples[i].0.clone();
				let c2 = self.samples[i + 1].0.clone();

				let f = (value.clone() - t1.clone()) / (t2 - t1);

				return c1 * f.clone() + c2 * (T::one() - f);
			}
		}

		return self.samples[self.samples.len() - 1].0.clone();
	}
}

