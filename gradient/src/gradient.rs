
use std::ops::*;
use std::vec::Vec;
use std::cmp::Ordering;
use std::fmt::Debug;

pub use num::traits::One;

#[derive(Clone, Deserialize, Serialize)]
pub struct Gradient<T: Sized + Clone, C: Sized + Clone> {
	samples : Vec<(C, T)>
}

impl<T, C> Gradient<T, C> 
	where C: Add<Output = C> + Mul<T, Output = C> + Sized + Clone + Debug,
	      T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One
		   + PartialOrd + Sized + Clone + Div<Output = T> + Debug
{
	pub fn new(samples : &[(C, T)]) -> Self {
		let mut vec : Vec<(C, T)> = samples.iter()
				.map(|&(ref a, ref b)| (a.clone(), b.clone()))
				.collect();

		vec.sort_by(|ref a, ref b| 
			if (*a).1 < (*b).1 { 
				Ordering::Less 
			} 
			else { 
				Ordering::Greater 
			});
			
		Self { 
			samples: vec
		}
	}

	pub fn value_at(&self, value : T) -> C {
		if self.samples[0].1 > value {
			return self.samples[0].0.clone();
		}

		for i in 0..(self.samples.len() - 1) {
			if self.samples[i + 1].1 > value {
				let t1 = self.samples[i].1.clone();
				let t2 = self.samples[i + 1].1.clone();

				let c1 = self.samples[i].0.clone();
				let c2 = self.samples[i + 1].0.clone();

				let f = (value.clone() - t1.clone()) / (t2 - t1);

				return c1 * (T::one() - f.clone()) + c2 * f;
			}
		}

		return self.samples[self.samples.len() - 1].0.clone();
	}
}

