use traits::*;
use functions::clamp;

use std::ops::*;

macro_rules! implement_functions {
	($type:ty) => {
		impl HasDot for $type {
			type Output = Self;

			fn dot(&self, rhs: Self) -> Self {
				self * rhs
			}
		}
		impl HasSqrt for $type {
			fn sqrt(&self) -> Self {
				(*self).sqrt()
			}

			fn inv_sqrt(&self) -> Self {
				self.sqrt().recip()
			}
		}
		impl HasAbs for $type {
			fn abs(&self) -> Self {
				(*self).abs()
			}
		}
		impl HasTrig for $type {
			fn sin(&self) -> Self {
				(*self).sin()
			}
			fn cos(&self) -> Self {
				(*self).cos()
			}
			fn tan(&self) -> Self {
				(*self).tan()
			}

			fn asin(&self) -> Self {
				(*self).asin()
			}
			fn acos(&self) -> Self {
				(*self).acos()
			}
			fn atan(&self) -> Self {
				(*self).atan()
			}
		}
		impl HasExponential for $type {
			fn pow(&self, exp: Self) -> Self {
				self.powf(exp)
			}
			fn log(&self) -> Self {
				(*self).ln()
			}
			fn exp(&self) -> Self {
				(*self).exp()
			}
			fn log2(&self) -> Self {
				(*self).log2()
			}
			fn exp2(&self) -> Self {
				(*self).exp2()
			}
		}
		impl HasSign for $type {
			fn sign(&self) -> Self {
				if *self < 0.0 { return -1.0; }
				if *self > 0.0 { return 1.0;  }
				return 0.0;
			}
		}
		impl HasFloor for $type {
			fn floor(&self) -> Self {
				(*self).floor()
			}
		}
		impl HasCeil for $type {
			fn ceil(&self) -> Self {
				(*self).ceil()
			}
		}
		impl HasFract for $type {
			fn fract(&self) -> Self {
				(*self).fract()
			}
		}
		impl HasMinMax for $type {
			fn min(&self, rhs: Self) -> Self {
				(*self).min(rhs)
			}
			fn max(&self, rhs: Self) -> Self {
				(*self).max(rhs)
			}
		}
		impl HasMix for $type {
			type ElemType = Self;

			fn mix(&self, rhs: Self, f: Self) -> Self {
				*self * f + rhs * (1.0 - f)
			}
		}
		impl HasStep for $type {
			fn step(&self, edge: Self) -> Self {
				if *self < edge { 0.0 } else { 1.0 }
			}
		}
		impl HasSmoothStep for $type {
			fn smoothstep(&self, edge0: Self, edge1: Self) -> Self {
				let x = clamp((*self - edge0) / (edge1 - edge0), 0.0, 1.0);

				x * x * (3.0 - 2.0 * x)
			}
		}

		impl Zero for $type {
			fn zero() -> Self {
				0.0
			}
		}
		impl One for $type {
			fn one() -> Self {
				1.0
			}
		}

		impl HasClamp for $type {
			type ElemType = Self;

			fn clamp(&self, min: Self, max: Self) -> Self {
				self.min(max).max(min)
			}
		}
	}
}

implement_functions!(f32);
implement_functions!(f64);

impl<T: HasPerElementBinOps> HasMinMax for T
where
    T::ElemType: HasMinMax,
{
    fn min(&self, rhs: Self) -> Self {
        self.apply_bin_op(rhs, |ref a, b| a.min(b))
    }
    fn max(&self, rhs: Self) -> Self {
        self.apply_bin_op(rhs, |ref a, b| a.max(b))
    }
}
impl<T: HasPerElementBinOps> HasStep for T
where
    T::ElemType: HasStep,
{
    fn step(&self, rhs: Self) -> Self {
        self.apply_bin_op(rhs, |ref a, b| a.step(b))
    }
}

impl<T> HasDistance for T
where
    T: Sub<T> + Clone,
    T::Output: HasLength,
{
    type Output = <<T as Sub<T>>::Output as HasLength>::Output;

    fn distance(&self, rhs: Self) -> Self::Output {
        (self.clone() - rhs).length()
    }
}
impl<T> HasNormalize for T
where
    T: HasLength + Div<<T as HasLength>::Output, Output = T> + Clone,
{
    fn normalize(&self) -> Self {
        self.clone() / self.length()
    }
}

impl<T: HasPerElementBinOps> HasMix for T
where
    T::ElemType: HasMix<ElemType = T::ElemType> + Clone,
{
    type ElemType = T::ElemType;

    fn mix(&self, b: Self, f: Self::ElemType) -> T {
        self.apply_bin_op(b, move |ref a, b| a.mix(b, f.clone()))
    }
}
