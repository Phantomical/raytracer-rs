
use prelude::*;
use vecmacro::Vec3;

use std::ops::*;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Colour<T>(Vec3<T>)
	where T: Field;

impl<T: Field> Colour<T> {
	pub const fn new(r : T, g : T, b : T) -> Colour<T> {
		return Colour{ 0: Vec3::new([r,g,b]) };
	}
	pub fn vec_new(v : Vec3<T>) -> Colour<T> {
		return Colour{ 0: v };
	}
}

impl<T: Field> Mul<T> for Colour<T> {
	type Output = Colour<<T as Mul>::Output>;

	fn mul(self, v : T) -> Self::Output {
		return Colour::vec_new(self.0 * v);
	}
}
impl<T: Field> Add for Colour<T> {
	type Output = Colour<<T as Add>::Output>;

	fn add(self, rhs : Self) -> Self::Output {
		return Colour::vec_new(self.0 + rhs.0);
	}
}
impl<T: Field> Sub for Colour<T> 
	where <T as Sub>::Output: Field
{
	type Output = Colour<<T as Sub>::Output>;

	fn sub(self, rhs : Self) -> Self::Output {
		return Colour::vec_new(self.0 - rhs.0);
	}
}

impl<T: Field> Mul for Colour<T> {
	type Output = Colour<<T as Mul>::Output>;

	fn mul(self, rhs : Self) -> Self::Output {
		return Colour::new(
			self.0.data[0] * rhs.0.data[0],
			self.0.data[1] * rhs.0.data[1],
			self.0.data[2] * rhs.0.data[2]);
	}
}

impl<T: Field> VectorSpace<T> for Colour<T> {}
impl<T: Field> VectorField<T> for Colour<T>
	where <T as Sub>::Output: Field {}
