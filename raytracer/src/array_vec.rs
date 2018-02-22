
mod internal {
	use ndarray::*;
	use glslvec::prelude::*;
	use std::ops::*;
	use std::convert::From;

	use vec::{Vec3, vec3};
	
	#[derive(Clone)]
	pub struct ArrayVec3<T, D>
		where D: Dimension
	{
		pub x: Array<T, D>,
		pub y: Array<T, D>,
		pub z: Array<T, D>
	}
	
	impl<T, D> ArrayVec3<T, D>
		where D: Dimension
	{
		pub fn new(
			x: Array<T ,D>,
			y: Array<T ,D>, 
			z: Array<T ,D>) -> Self 
		{
			Self { x, y, z }
		}

		pub fn elem_size(&self) -> usize {
			self.x.shape().iter().product()
		}
	}

	impl<T> From<Vec3<T>> for ArrayVec3<T, IxDyn> {
		fn from(v: Vec3<T>) -> Self {
			Self {
				x: array![v.x].into_dyn(),
				y: array![v.y].into_dyn(),
				z: array![v.z].into_dyn()
			}
		}
	}
	
	macro_rules! bin_op {
		($trait:ident, $op:tt, $name:ident, $T:ty) => {
			impl<'a, S, D> $trait<&'a ArrayBase<S, D>> for ArrayVec3<$T, D> 
				where S: Data<Elem = $T>,
				      D: Dimension
			{
				type Output = ArrayVec3<$T, D>;
			
				fn $name(self, rhs: &ArrayBase<S, D>) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						self.x $op rhs,
						self.y $op rhs,
						self.z $op rhs
					)
				}
			}

			impl<'a, 'b, D> $trait<&'b Array<$T, D>> for &'a ArrayVec3<$T, D> 
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;
			
				fn $name(self, rhs: &Array<$T, D>) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						&self.x $op rhs,
						&self.y $op rhs,
						&self.z $op rhs
					)
				}
			}

			impl<'a, D> $trait<Array<$T, D>> for &'a ArrayVec3<$T, D> 
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;
			
				fn $name(self, rhs: Array<$T, D>) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						&self.x $op &rhs,
						&self.y $op &rhs,
						&self.z $op &rhs
					)
				}
			}
			impl<'a, 'b, D> $trait<&'b ArrayVec3<$T, D>> for &'a ArrayVec3<$T, D> 
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;
			
				fn $name(self, rhs: &ArrayVec3<$T, D>) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						&self.x $op &rhs.x,
						&self.y $op &rhs.y,
						&self.z $op &rhs.z
					)
				}
			}

			impl<'a, S, D> $trait<ArrayBase<S, D>> for ArrayVec3<$T, D> 
				where S: Data<Elem = $T>,
					  D: Dimension
			{
				type Output = ArrayVec3<$T, D>;
		
				fn $name(self, rhs: ArrayBase<S, D>) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						self.x $op &rhs,
						self.y $op &rhs,
						self.z $op rhs
					)
				}
			}

			impl<'a, D> $trait<ArrayVec3<$T, D>> for &'a ArrayVec3<$T, D> 
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;
			
				fn $name(self, rhs: ArrayVec3<$T, D>) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						&self.x $op &rhs.x,
						&self.y $op &rhs.y,
						&self.z $op &rhs.z
					)
				}
			}

			impl<D> $trait<$T> for ArrayVec3<$T, D>
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;

				fn $name(self, rhs: $T) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						self.x $op rhs,
						self.y $op rhs,
						self.z $op rhs
					)
				}
			}
			impl<'a, D> $trait<$T> for &'a ArrayVec3<$T, D>
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;

				fn $name(self, rhs: $T) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						&self.x $op rhs,
						&self.y $op rhs,
						&self.z $op rhs
					)
				}
			}

			impl<D> $trait for ArrayVec3<$T, D>
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;

				fn $name(self, rhs: ArrayVec3<$T, D>) -> ArrayVec3<$T, D> {
					ArrayVec3::new(
						self.x $op rhs.x,
						self.y $op rhs.y,
						self.z $op rhs.z
					)
				}
			}

			impl<'a, D> $trait<&'a ArrayVec3<$T, D>> for ArrayVec3<$T, D>
				where D: Dimension
			{
				type Output = ArrayVec3<$T, D>;

				fn $name(self, rhs: &ArrayVec3<$T, D>) -> Self::Output {
					ArrayVec3::new(
						self.x $op &rhs.x,
						self.y $op &rhs.y,
						self.z $op &rhs.z
					)
				}
			}
		}
	}

	macro_rules! impl_traits {
		($type:tt) => {
			bin_op!(Add, +, add, $type);
			bin_op!(Sub, -, sub, $type);
			bin_op!(Mul, *, mul, $type);
			bin_op!(Div, /, div, $type);
	
			impl<D> HasDot for ArrayVec3<$type, D>
			where D: Dimension
			{
				type Output = Array<$type, D>;

				fn dot(&self, rhs: Self) -> Self::Output {
					return rhs.x * &self.x 
						 + rhs.y * &self.y
						 + rhs.z * &self.z;
				}
			}
			impl<D> HasSqrt for ArrayVec3<$type, D> 
			where D: Dimension
			{
				fn sqrt(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::sqrt),
						self.y.mapv($type::sqrt),
						self.z.mapv($type::sqrt))
				}
				fn inv_sqrt(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv(|x| x.sqrt().recip()),
						self.y.mapv(|x| x.sqrt().recip()),
						self.z.mapv(|x| x.sqrt().recip())
					)
				}
			}
			impl<D> HasCross for ArrayVec3<$type, D> 
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
			impl<D> HasLength for ArrayVec3<$type, D>
			where D: Dimension
			{
				type Output = Array<$type, D>;

				fn length(&self) -> Self::Output {
					(&self.x * &self.x + &self.y * &self.y + &self.z * &self.z)
						.mapv($type::sqrt)
				}
			}
			impl<D> HasAbs for ArrayVec3<$type, D> 
			where D: Dimension
			{
				fn abs(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::abs),
						self.y.mapv($type::abs),
						self.z.mapv($type::abs))
				}
			}
			impl<D> HasTrig for ArrayVec3<$type, D> 
			where D: Dimension
			{
				fn sin(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::sin),
						self.y.mapv($type::sin),
						self.z.mapv($type::sin))
				}
				fn cos(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::cos),
						self.y.mapv($type::cos),
						self.z.mapv($type::cos))
				}
				fn tan(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::tan),
						self.y.mapv($type::tan),
						self.z.mapv($type::tan))
				}
				fn asin(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::asin),
						self.y.mapv($type::asin),
						self.z.mapv($type::asin))
				}
				fn acos(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::acos),
						self.y.mapv($type::acos),
						self.z.mapv($type::acos))
				}
				fn atan(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::atan),
						self.y.mapv($type::atan),
						self.z.mapv($type::atan))
				}
			}
			impl<D> HasFloor for ArrayVec3<$type, D>
			where D: Dimension
			{
				fn floor(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::floor),
						self.y.mapv($type::floor),
						self.z.mapv($type::floor))
				}
			}
			impl<D> HasCeil for ArrayVec3<$type, D>
			where D: Dimension
			{
				fn ceil(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::ceil),
						self.y.mapv($type::ceil),
						self.z.mapv($type::ceil))
				}
			}
			impl<D> HasFract for ArrayVec3<$type, D>
			where D: Dimension
			{
				fn fract(&self) -> Self {
					ArrayVec3::new(
						self.x.mapv($type::fract),
						self.y.mapv($type::fract),
						self.z.mapv($type::fract))
				}
			}
			impl<D> HasMinMax for ArrayVec3<$type, D>
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
	}

	impl_traits!(f64);
	impl_traits!(f32);

	impl<D, T> ArrayVec3<T, D> 
		where D: Dimension,
		      T: Clone
	{
		pub fn at<Idx>(&self, idx: Idx) -> Vec3<T>
			where Idx: NdIndex<D> + Copy
		{
			vec3(
				self.x[idx].clone(),
				self.y[idx].clone(),
				self.z[idx].clone()
			)
		}
	}
}

use ndarray::IxDyn;

pub type ArrayVec3 = internal::ArrayVec3<f64, IxDyn>;
pub type ArrayColour = internal::ArrayVec3<f32, IxDyn>;
