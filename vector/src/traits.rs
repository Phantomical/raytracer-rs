
use std::ops::*;
use num::traits::{Zero, One};

pub trait Ring: Add + Mul + Zero + One + Sized + Copy + Default {}
pub trait Field: Ring + Sub + Div + Neg {}
pub trait VectorSpace<T: Ring>: Add + Mul<T> + Sized + Copy {}

pub trait Dottable {
	type Result;

	fn dot(&self, rhs : &Self) -> Self::Result;
}
pub trait Normalizable {
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

