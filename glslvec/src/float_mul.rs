
use vec2::*;
use vec3::*;
use vec4::*;

use std::ops::*;

macro_rules! add_mul {
	($type:ty, $vecname:ident) => {
		impl Add<$vecname<$type>> for $type {
			type Output = $vecname<$type>;
		
			fn add(self, rhs: $vecname<$type>) -> $vecname<$type> {
				rhs + self
			}
		}
		impl Mul<$vecname<$type>> for $type {
			type Output = $vecname<$type>;
		
			fn mul(self, rhs: $vecname<$type>) -> $vecname<$type> {
				rhs * self
			}
		}
	}
}

macro_rules! add_mul_all {
	($type:ty) => {
		add_mul!($type, Vec2);
		add_mul!($type, Vec3);
		add_mul!($type, Vec4);
	}
}

add_mul_all!(f32);
add_mul_all!(f64);
