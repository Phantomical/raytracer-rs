
use std::ops::Mul;

trait InternalMul<T>: Mul<T> {}

macro_rules! implement_vector_mul {
	($name:ident, $T:ty) => {
		impl Mul<$name<$T>> for $T {
			type Output = <$name<$T> as Mul<$T>>::Output;

			fn mul(self, rhs: $name<$T>) -> Self::Output {
				return rhs * self;
			}
		}
	}
}
macro_rules! implement_fixed_vector {
	($namespace:ident, $name:ident, $dims:expr) => { 
		mod $namespace {
			use prelude::*;
			use std::ops::*;
			use std::iter::*;
			use num::traits::Zero;

			#[derive(Default, Copy, Clone)]
			pub struct $name<T: Sized + Copy + Default> {
				pub data : [T; $dims]
			}

			impl<T: Sized + Copy + Default> $name<T> {
				pub fn new(dat : [T; $dims]) -> Self {
					return Self {
						data: dat
					};
				}
				pub fn single_element(elem : T) -> Self {
					return Self {
						data: [elem; $dims]
					};
				}
			}

			impl<T: Ring> Add for $name<T>
				where <T as Add>::Output: Ring
			{
				type Output = $name<<T as Add>::Output>;

				fn add(self, rhs: Self) -> Self::Output {
					let mut vec = Self::Output::default();
					for i in 0..$dims{
						vec.data[i] = self.data[i] + rhs.data[i];
					}
					return vec;
				}
			}
			impl<T: Field> Sub for $name<T>
				where <T as Sub>::Output: Ring
			{
				type Output = $name<<T as Sub>::Output>;
				
				fn sub(self, rhs: Self) -> Self::Output {
					let mut vec = Self::Output::default();
					for i in 0..$dims{
						vec.data[i] = self.data[i] - rhs.data[i];
					}
					return vec;
				}
			}
			impl<T: Ring> Mul<T> for $name<T> 
				where <T as Mul>::Output: Ring
			{
				type Output = $name<<T as Mul>::Output>;

				fn mul(self, rhs: T) -> Self::Output {
					let mut vec = Self::Output::default();
					for i in 0..$dims{
						vec.data[i] = self.data[i] * rhs;
					}
					return vec;
				}
			}

			impl<T: PartialEq + Sized + Copy + Default> PartialEq for $name<T> {
				fn eq(&self, other: &Self) -> bool {
					return self.data == other.data;
				}
			}
			impl<T: Eq + Sized + Copy + Default> Eq for $name<T> {}

			impl<T: Ring> VectorSpace<T> for $name<T> {}
			impl<T: Field> VectorField<T> for $name<T>
				where <T as Sub>::Output: Ring {}

			impl<T: Ring> Zero for $name<T> {
				fn zero() -> Self {
					return Self::single_element(T::zero());
				}
				default fn is_zero(&self) -> bool {
					unimplemented!();
				}
			}
			impl<T: Ring + PartialEq> Zero for $name<T> {
				fn is_zero(&self) -> bool {
					return self.data.iter()
						.all(|x| *x == T::zero());
				}
			}

			impl<T: Ring> Dottable for $name<T> {
				type Output = T;

				fn dot(&self, rhs: &Self) -> Self::Output {
					return self.data.iter()
						.zip(rhs.data.iter())
						.map(|(x,y)| *x * *y)
						.fold(T::zero(), |a, x : T| a + x);
				}
			}

			impl<T: Field> Div<T> for $name<T>
				where <T as Div>::Output: Ring
			{
				type Output = $name<<T as Div>::Output>;

				fn div(self, rhs: T) -> Self::Output {
					let mut vec = Self::Output::default();
					for i in 0..$dims {
						vec.data[i] = self.data[i] / rhs;
					}
					return vec;
				}
			}

			impl<T: Field + Sqrt> Normalizable for $name<T> 
				where $name<T>: Div<T, Output = $name<T>>
			{
				type Output = <$name<T> as Dottable>::Output;

				fn magnitude(&self) -> T {
					return self.dot(self).sqrt();
				}
				fn normalized(&self) -> $name<T> {
					return *self / self.magnitude();
				}
			}
		}

		pub type $name<T> = $namespace::$name<T>;

		implement_vector_mul!($name, f32);
		implement_vector_mul!($name, f64);

		implement_vector_mul!($name, u8);
		implement_vector_mul!($name, u16);
		implement_vector_mul!($name, u32);
		implement_vector_mul!($name, u64);

		implement_vector_mul!($name, i8);
		implement_vector_mul!($name, i16);
		implement_vector_mul!($name, i32);
		implement_vector_mul!($name, i64);
	}
}

implement_fixed_vector!(vec2, Vec2, 2);
implement_fixed_vector!(vec3, Vec3, 3);
implement_fixed_vector!(vec4, Vec4, 4);
//implement_fixed_vector!(vec5, Vec5, 5);
//implement_fixed_vector!(vec6, Vec6, 6);
//implement_fixed_vector!(vec7, Vec7, 7);
//implement_fixed_vector!(vec8, Vec8, 8);
//implement_fixed_vector!(vec9, Vec9, 9);
//implement_fixed_vector!(vec10, Vec10, 10);


use prelude::{HasX, HasY, HasZ, HasW};

impl<T: Copy + Clone + Sized + Default> HasX<T> for Vec2<T> {
	fn x(&self) -> T {
		return self.data[0];
	} 
}
impl<T: Copy + Clone + Sized + Default> HasY<T> for Vec2<T> {
	fn y(&self) -> T {
		return self.data[1];
	}
}

impl<T: Copy + Clone + Sized + Default> HasX<T> for Vec3<T> {
	fn x(&self) -> T {
		return self.data[0];
	}
}
impl<T: Copy + Clone + Sized + Default> HasY<T> for Vec3<T> {
	fn y(&self) -> T {
		return self.data[1];
	}
}
impl<T: Copy + Clone + Sized + Default> HasZ<T> for Vec3<T> {
	fn z(&self) -> T {
		return self.data[2];
	}
}

impl<T: Copy + Clone + Sized + Default> HasX<T> for Vec4<T> {
	fn x(&self) -> T {
		return self.data[0];
	}
}
impl<T: Copy + Clone + Sized + Default> HasY<T> for Vec4<T> {
	fn y(&self) -> T {
		return self.data[1];
	}
}
impl<T: Copy + Clone + Sized + Default> HasZ<T> for Vec4<T> {
	fn z(&self) -> T {
		return self.data[2];
	}
}
impl<T: Copy + Clone + Sized + Default> HasW<T> for Vec4<T> {
	fn w(&self) -> T {
		return self.data[2];
	}
}
