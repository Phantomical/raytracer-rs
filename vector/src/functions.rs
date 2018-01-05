
use prelude::*;
use vecmacro::Vec3;
use std::ops::Sub;

pub fn dot<V: Dottable>(lhs: &V, rhs: &V) -> V::Result
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