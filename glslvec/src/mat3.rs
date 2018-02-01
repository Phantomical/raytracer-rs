
use vec3::*;
use traits::*;
use functions::dot;

use std::ops::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Mat3<T: Sized> {
	pub rows: [Vec3<T>; 3]
}

impl<T: Sized + Clone> Mat3<T> {
	pub fn new(vals: [T; 9]) -> Self {
		Self { rows: [
			vec3(vals[0].clone(), vals[1].clone(), vals[2].clone()),
			vec3(vals[3].clone(), vals[4].clone(), vals[5].clone()),
			vec3(vals[6].clone(), vals[7].clone(), vals[8].clone()) ]	
		}
	}

	pub fn from_vecs(rows: [Vec3<T>; 3]) -> Self {
		Self { rows	}
	}
}

impl<T> Mat3<T>
	where T: Sized + Copy + HasTrig + One + Mul<Output = T> 
	       + Sub<Output = T> + Add<Output = T>,
		  Vec3<T>: HasNormalize
{
	pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Mat3<T> {
		let sin = angle.sin();
		let cos = angle.cos();
		let u = axis.normalize();

		let r1 = vec3(
			cos + u.x * u.x * (T::one() - cos),
			u.x * u.y * (T::one() - cos) - u.z * sin,
			u.x * u.z * (T::one() - cos) + u.y *  sin);
		let r2 = vec3(
			u.y * u.x * (T::one() - cos) + u.z * sin,
			cos + u.y * u.y * (T::one() - cos),
			u.y * u.z * (T::one() - cos) - u.x * sin);
		let r3 = vec3(
			u.z * u.x * (T::one() - cos) - u.y * sin,
			u.x * u.y * (T::one() - cos) + u.x * sin,
			cos + u.z * u.z * (T::one() - cos));

		Mat3::from_vecs([ r1, r2, r3 ])
	}
}

impl<T> Mat3<T>
	where T: Copy + Sized + Mul<Output = T> + Sub<Output = T> 
		+ Add<Output = T> + One + Div<Output = T>
{
	pub fn inverse(&self) -> Self {
		let invdet = T::one() / (
			self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
			self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0]) +
			self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0]));

		Mat3::new([
			(self[1][1] * self[2][2] - self[2][1] * self[1][2]) * invdet,
			(self[0][2] * self[2][1] - self[0][1] * self[2][2]) * invdet,
			(self[0][1] * self[1][2] - self[0][2] * self[1][1]) * invdet,
			(self[1][2] * self[2][0] - self[1][0] * self[2][2]) * invdet,
			(self[0][0] * self[2][2] - self[0][2] * self[2][0]) * invdet,
			(self[1][0] * self[0][2] - self[0][0] * self[1][2]) * invdet,
			(self[1][0] * self[2][1] - self[2][0] * self[1][1]) * invdet,
			(self[2][0] * self[0][1] - self[0][0] * self[2][1]) * invdet,
			(self[0][0] * self[1][1] - self[1][0] * self[0][1]) * invdet
		])
	}
}

impl<T> Index<usize> for Mat3<T>
	where T: Sized
{
	type Output = Vec3<T>;

	fn index(&self, idx: usize) -> &Vec3<T> {
		&self.rows[idx]
	}
}

impl<T> Mul for Mat3<T> 
	where T: Copy + Sized + Mul<Output = T> + Add<Output = T>
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		let cols = [
			vec3(rhs[0][0], rhs[1][0], rhs[2][0]),
			vec3(rhs[0][1], rhs[1][1], rhs[2][1]),
			vec3(rhs[0][2], rhs[1][2], rhs[2][2]) ];

		return Mat3::new([
			dot(self[0], cols[0]), dot(self[0], cols[1]), dot(self[0], cols[2]),
			dot(self[1], cols[0]), dot(self[1], cols[1]), dot(self[1], cols[2]),
			dot(self[2], cols[0]), dot(self[2], cols[1]), dot(self[2], cols[2]) 
		]);
	}
}

impl<T> Mul<Vec3<T>> for Mat3<T>
	where T: Copy + Sized + Mul<Output = T> + Add<Output = T>
{
	type Output = Vec3<T>;

	fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
		vec3(dot(self[0], rhs), dot(self[1], rhs),dot(self[2], rhs))
	}
}
