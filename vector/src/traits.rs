
use std::ops::*;
use num::traits::{Zero, One};

pub trait Ring: Add + Mul + Zero + One + Sized + Copy + Default {}
pub trait Field: Ring + Sub + Div + Neg {}
pub trait VectorSpace<T: Ring>: Add + Mul<T> + Sized + Copy {}
pub trait VectorField<T: Field>: VectorSpace<T> + Sub {}

pub trait Trig: Sized + Copy {
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;

	fn sin_cos(self) -> (Self, Self) {
		return (self.sin(), self.cos());
	}
}
pub trait Sqrt {
	fn sqrt(self) -> Self;
}

pub trait Dottable {
	type Output;

	fn dot(&self, rhs : &Self) -> Self::Output;
}
pub trait Normalizable {
	type Output;

	fn magnitude(&self) -> Self::Output;
	fn normalized(&self) -> Self;
}

pub trait HasX<T> {
	fn x(&self) -> T;
}
pub trait HasY<T>: HasX<T> {
	fn y(&self) -> T;
}
pub trait HasZ<T>: HasY<T> {
	fn z(&self) -> T;
}
pub trait HasW<T>: HasZ<T> {
	fn w(&self) -> T;
}


impl Ring for u8 {}
impl Ring for u16 {}
impl Ring for u32 {}
impl Ring for u64 {}

impl Ring for i8 {}
impl Ring for i16 {}
impl Ring for i32 {}
impl Ring for i64 {}

impl Ring for f32 {}
impl Ring for f64 {}

impl Field for f32 {}
impl Field for f64 {}

impl Trig for f32 {
	fn sin(self) -> Self {
		return self.sin();
	}
	fn cos(self) -> Self {
		return self.cos();
	}
	fn tan(self) -> Self {
		return self.tan();
	}
	fn sin_cos(self) -> (Self, Self) {
		return self.sin_cos();
	}
}
impl Trig for f64 {
	fn sin(self) -> Self {
		return self.sin();
	}
	fn cos(self) -> Self {
		return self.cos();
	}
	fn tan(self) -> Self {
		return self.tan();
	}
	fn sin_cos(self) -> (Self, Self) {
		return self.sin_cos();
	}
}

impl Sqrt for f32 {
	fn sqrt(self) -> Self {
		return self.sqrt();
	}
}
impl Sqrt for f64 {
	fn sqrt(self) -> Self {
		return self.sqrt();
	}
}


