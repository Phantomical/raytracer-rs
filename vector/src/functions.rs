
use prelude::*;
use vecmacro::Vec3;
use std::ops::{Sub, Div, Mul};

pub fn dot<V: Dottable>(lhs: &V, rhs: &V) -> V::Output
{
	return lhs.dot(rhs);
}
pub fn normalize<V: Normalizable>(val: &V) -> V {
	return val.normalized();
}

pub fn cross<T: Field>(u: Vec3<T>, v: Vec3<T>) -> Vec3<<T as Sub>::Output>
	where <T as Sub>::Output: Sized + Default + Copy
{
	return Vec3::new([
		u.y() * v.z() - u.z() * v.y(),
		u.z() * v.x() - u.x() * v.z(),
		u.x() * v.y() - v.y() * u.x()
	]);
}

/// Calculates the projection of x onto y.
/// In mathematical notation this would be
/// written as proj_y (x).
/// 
/// The projection of x onto y is the component
/// of x that is parallel to y.
/// 
/// # Example
/// ```
/// let x = Vec3d::new(1.0, 1.0, 0.0);
/// let y = Vec3d::new(1.0, 0.0, 0.0);
/// 
/// assert_eq!(proj(x, y), y);
/// ```
///
pub fn proj<T: Dottable + Normalizable + Clone>(x : &T, y : &T) -> 
	<<<T as Dottable>::Output as Div>::Output as Mul<T>>::Output
	where <T as Dottable>::Output: Div,
	      <<T as Dottable>::Output as Div>::Output: Mul<T>
{
	return (dot(x, y) / dot(y, y)) * (*y).clone();
}

pub fn perp<T: Dottable + Normalizable + Clone>(x : &T, y : &T) -> 
	<T as Sub<<<<T as Dottable>::Output as Div>::Output as Mul<T>>::Output>>::Output
	where <T as Dottable>::Output: Div,
	      <<T as Dottable>::Output as Div>::Output: Mul<T>,
		  <<<T as Dottable>::Output as Div>::Output as Mul<T>>::Output: Sub,
		  T: Sub<<<<T as Dottable>::Output as Div>::Output as Mul<T>>::Output, Output = T>
{
	return (*x).clone() - proj(x, y);
}

/// Rotates a vector around an axis by an angle (in radians)
/// Source: https://math.stackexchange.com/questions/511370/how-to-rotate-one-vector-about-another
pub fn rotate<T>(v : &Vec3<T>, axis : &Vec3<T>, angle : T) -> Vec3<T> 
	where T: Field + Mul<Vec3<T>, Output = Vec3<T>> + Sqrt 
	         + Div<Output = T> + Trig + Sub<Output = T>,
	      Vec3<T>: Sub<Output = Vec3<T>>
{
	let par : Vec3<T> = proj(v, axis);
	let perp = *v - par;

	let w = cross(*axis, perp).normalized();
	let pmag = perp.magnitude();

	let f = angle.sin_cos();	
	let rot = f.1 * perp + pmag * f.0 * w;

	return rot + par;
}