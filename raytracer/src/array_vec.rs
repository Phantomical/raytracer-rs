
mod internal {
	use ndarray::*;
	use glslvec::prelude::*;
	use std::ops::*;
	use std::convert::From;

	use vec::Vec3d;
	
	pub struct ArrayVec3<D>
		where D: Dimension
	{
		pub x: Array<f64, D>,
		pub y: Array<f64, D>,
		pub z: Array<f64, D>
	}
	
	impl<D> ArrayVec3<D>
		where D: Dimension
	{
		pub fn new(
			x: Array<f64 ,D>,
			y: Array<f64 ,D>, 
			z: Array<f64 ,D>) -> Self 
		{
			Self { x, y, z }
		}
	}

	impl From<Vec3d> for ArrayVec3<IxDyn> {
		fn from(v: Vec3d) -> Self {
			Self {
				x: array![v.x].into_dyn(),
				y: array![v.y].into_dyn(),
				z: array![v.z].into_dyn()
			}
		}
	}
	
	macro_rules! bin_op {
		($trait:ident, $op:tt, $name:ident) => {
			impl<'a, S, D> $trait<&'a ArrayBase<S, D>> for ArrayVec3<D> 
				where S: Data<Elem = f64>,
				      D: Dimension
			{
				type Output = ArrayVec3<D>;
			
				fn $name(self, rhs: &ArrayBase<S, D>) -> ArrayVec3<D> {
					ArrayVec3::new(
						self.x $op rhs,
						self.y $op rhs,
						self.z $op rhs
					)
				}
			}

			impl<'a, 'b, D> $trait<&'b Array<f64, D>> for &'a ArrayVec3<D> 
				where D: Dimension
			{
				type Output = ArrayVec3<D>;
			
				fn $name(self, rhs: &Array<f64, D>) -> ArrayVec3<D> {
					ArrayVec3::new(
						&self.x $op rhs,
						&self.y $op rhs,
						&self.z $op rhs
					)
				}
			}

			impl<'a, D> $trait<Array<f64, D>> for &'a ArrayVec3<D> 
				where D: Dimension
			{
				type Output = ArrayVec3<D>;
			
				fn $name(self, rhs: Array<f64, D>) -> ArrayVec3<D> {
					ArrayVec3::new(
						&self.x $op &rhs,
						&self.y $op &rhs,
						&self.z $op &rhs
					)
				}
			}
			impl<'a, 'b, D> $trait<&'b ArrayVec3<D>> for &'a ArrayVec3<D> 
				where D: Dimension
			{
				type Output = ArrayVec3<D>;
			
				fn $name(self, rhs: &ArrayVec3<D>) -> ArrayVec3<D> {
					ArrayVec3::new(
						&self.x $op &rhs.x,
						&self.y $op &rhs.y,
						&self.z $op &rhs.z
					)
				}
			}

			impl<'a, S, D> $trait<ArrayBase<S, D>> for ArrayVec3<D> 
				where S: Data<Elem = f64>,
					  D: Dimension
			{
				type Output = ArrayVec3<D>;
		
				fn $name(self, rhs: ArrayBase<S, D>) -> ArrayVec3<D> {
					ArrayVec3::new(
						self.x $op &rhs,
						self.y $op &rhs,
						self.z $op rhs
					)
				}
			}

			impl<'a, D> $trait<ArrayVec3<D>> for &'a ArrayVec3<D> 
				where D: Dimension
			{
				type Output = ArrayVec3<D>;
			
				fn $name(self, rhs: ArrayVec3<D>) -> ArrayVec3<D> {
					ArrayVec3::new(
						&self.x $op &rhs.x,
						&self.y $op &rhs.y,
						&self.z $op &rhs.z
					)
				}
			}

			impl<D> $trait<f64> for ArrayVec3<D>
				where D: Dimension
			{
				type Output = ArrayVec3<D>;

				fn $name(self, rhs: f64) -> ArrayVec3<D> {
					ArrayVec3::new(
						self.x $op rhs,
						self.y $op rhs,
						self.z $op rhs
					)
				}
			}
			impl<'a, D> $trait<f64> for &'a ArrayVec3<D>
				where D: Dimension
			{
				type Output = ArrayVec3<D>;

				fn $name(self, rhs: f64) -> ArrayVec3<D> {
					ArrayVec3::new(
						&self.x $op rhs,
						&self.y $op rhs,
						&self.z $op rhs
					)
				}
			}
		}
	}
	
	bin_op!(Add, +, add);
	bin_op!(Sub, -, sub);
	bin_op!(Mul, *, mul);
	bin_op!(Div, /, div);
	
	impl<D> HasDot for ArrayVec3<D>
	where D: Dimension
	{
		type Output = Array<f64, D>;

		fn dot(&self, rhs: Self) -> Self::Output {
			return rhs.x * &self.x 
			     + rhs.y * &self.y
			     + rhs.z * &self.z;
		}
	}
	impl<D> HasSqrt for ArrayVec3<D> 
	where D: Dimension
	{
		fn sqrt(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::sqrt),
				self.y.mapv(f64::sqrt),
				self.z.mapv(f64::sqrt))
		}
		fn inv_sqrt(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(|x| x.sqrt().recip()),
				self.y.mapv(|x| x.sqrt().recip()),
				self.z.mapv(|x| x.sqrt().recip())
			)
		}
	}
	impl<D> HasCross for ArrayVec3<D> 
	where D: Dimension 
	{
		fn cross(&self, rhs: Self) -> Self {
			let x1 = &rhs.z * &self.y;
			let y1 = &rhs.x * &self.z;
			let z1 = &rhs.y * &self.x;

			let x2 = rhs.y * &self.z;
			let y2 = rhs.z * &self.x;
			let z2 = rhs.x * &self.y;

			ArrayVec3::new(x1 - x2, y1 - y2, z1 - z2)
		}
	}
	impl<D> HasLength for ArrayVec3<D>
	where D: Dimension
	{
		type Output = Array<f64, D>;

		fn length(&self) -> Self::Output {
			(&self.x * &self.x + &self.y * &self.y + &self.z * &self.z)
				.mapv(f64::sqrt)
		}
	}
	impl<D> HasNormalize for ArrayVec3<D>
	where D: Dimension
	{
		fn normalize(&self) -> Self {
			let mult = 1.0 / self.length();

			self * mult
		}
	}
	impl<D> HasAbs for ArrayVec3<D> 
	where D: Dimension
	{
		fn abs(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::abs),
				self.y.mapv(f64::abs),
				self.z.mapv(f64::abs))
		}
	}
	impl<D> HasTrig for ArrayVec3<D> 
	where D: Dimension
	{
		fn sin(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::sin),
				self.y.mapv(f64::sin),
				self.z.mapv(f64::sin))
		}
		fn cos(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::cos),
				self.y.mapv(f64::cos),
				self.z.mapv(f64::cos))
		}
		fn tan(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::tan),
				self.y.mapv(f64::tan),
				self.z.mapv(f64::tan))
		}
		fn asin(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::asin),
				self.y.mapv(f64::asin),
				self.z.mapv(f64::asin))
		}
		fn acos(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::acos),
				self.y.mapv(f64::acos),
				self.z.mapv(f64::acos))
		}
		fn atan(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::atan),
				self.y.mapv(f64::atan),
				self.z.mapv(f64::atan))
		}
	}
	impl<D> HasFloor for ArrayVec3<D>
	where D: Dimension
	{
		fn floor(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::floor),
				self.y.mapv(f64::floor),
				self.z.mapv(f64::floor))
		}
	}
	impl<D> HasCeil for ArrayVec3<D>
	where D: Dimension
	{
		fn ceil(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::ceil),
				self.y.mapv(f64::ceil),
				self.z.mapv(f64::ceil))
		}
	}
	impl<D> HasFract for ArrayVec3<D>
	where D: Dimension
	{
		fn fract(&self) -> Self {
			ArrayVec3::new(
				self.x.mapv(f64::fract),
				self.y.mapv(f64::fract),
				self.z.mapv(f64::fract))
		}
	}
	impl<D> HasMinMax for ArrayVec3<D>
	where D: Dimension
	{
		fn min(&self, mut rhs: Self) -> Self {
			Zip::from(&mut rhs.x).and(&self.x).apply(|a, &b| *a = a.min(b));
			Zip::from(&mut rhs.y).and(&self.y).apply(|a, &b| *a = a.min(b));
			Zip::from(&mut rhs.z).and(&self.z).apply(|a, &b| *a = a.min(b));
			rhs
		}
		fn max(&self, mut rhs: Self) -> Self {
			Zip::from(&mut rhs.x).and(&self.x).apply(|a, &b| *a = a.max(b));
			Zip::from(&mut rhs.y).and(&self.y).apply(|a, &b| *a = a.max(b));
			Zip::from(&mut rhs.z).and(&self.z).apply(|a, &b| *a = a.max(b));
			rhs
		}
	}
}

use ndarray::IxDyn;

pub type ArrayVec3 = internal::ArrayVec3<IxDyn>;
