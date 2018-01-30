
use traits::*;
use std::ops::{Rem, Sub};

/// Returns the dot product of a and b. In
/// the case of scalars this returns a * b.
pub fn dot<T: HasDot>(a: T, b: T) -> T::Output {
	a.dot(b)
}
/// Returns the cross product between a and b
pub fn cross<T: HasCross>(a: T, b: T) -> T {
	a.cross(b)
}

/// Returns the length of the vector. In the
/// case of a scalar this returns the absolute
/// value.
pub fn length<T: HasLength>(a: T) -> T::Output {
	a.length()
}
/// Returns the square of the length of the vector.
pub fn length2<T: HasDot + Clone>(a: T) -> T::Output {
	dot(a.clone(), a)
}

/// Returns the distance between two points.
pub fn distance<T: HasDistance>(a: T, b: T) -> T::Output {
	a.distance(b)
}

/// Returns the square of the distance between two points
pub fn distance2<T>(a: T, b: T) -> <T as HasDot>::Output
	where T: HasDot + Sub<Output = T> + Clone
{
	let diff = a - b;
	dot(diff.clone(), diff)
}

/// Returns a vector with length 1 that is parallel
/// to a.
pub fn normalize<T: HasNormalize>(a: T) -> T {
	a.normalize()
}

/// Returns `n` if the dot product of `incident` and `nref` is less
/// than 0, `-n` otherwise.
pub fn faceforward<T: HasFaceForward>(n: T, incident: T, nref: T) -> T {
	n.faceforward(incident, nref)
}

/// Returns a vector that points in the direction that 
/// the reflection of `incident` off of a surface with
/// a given normal.
///
/// Note: The resulting vector will always have the 
/// same length as `incident`.
pub fn reflect<T: HasReflect>(incident: T, normal: T) -> T {
	incident.reflect(normal)
}

/// Calculates the refracted vector through a surface 
/// with the given normal and `eta` being the ratio of
/// indices of refraction.
pub fn refract<T: HasRefract>(incident: T, normal: T, eta: T::ElemType) -> T {
	incident.refract(normal, eta)
}

/// Returns the component-wise absolue value of a vector
pub fn abs<T: HasAbs>(x: T) -> T {
	x.abs()
}

/// Returns 1 when `x` is positive, -1 when `x` is negative,
/// and 0 otherwise. For vectors this operation is done
/// component-wise.
pub fn sign<T: HasSign>(x: T) -> T {
	x.sign()
}

/// Returns the largest integer that is less than or equal to
/// `x`. For vectors this operation is done component-wise.
pub fn floor<T: HasFloor>(x: T) -> T {
	x.floor()
}

/// Returns the smallest integer that is greater than or 
/// equal to `x`. For vectors this operation is done
/// component-wise.
pub fn ceil<T: HasCeil>(x: T) -> T {
	x.ceil()
}

/// Returns the fractional part of `x` (i.e. `x - floor(x)`).
/// For vectors this operation is done component-wise.
pub fn fract<T: HasFract>(x: T) -> T {
	x.fract()
}

/// Returns `x - y * floor(x / y)`. For vectors this is 
/// operation is done component-wise. This function is
/// equivalent to the `mod` function in GLSL.
///
/// Note: Implemented using the `%` operator.
pub fn modulus<T: Rem<Output = T>>(x: T, y: T) -> T {
	x % y
}

/// Returns the smaller of the two arguments. For vectors
/// this operation is done component-wise.
pub fn min<T: HasMinMax>(x: T, y: T) -> T {
	x.min(y)
}

/// Returns the larger of the two arguments. For vectors
/// this operation is done component-wise.
pub fn max<T: HasMinMax>(x: T, y: T) -> T {
	x.max(y)
}

/// Returns `x` is `x` is larger than `min`. If `x` 
/// is smaller than `min`, `min` is returned. If `x`
/// is larger than `max`, `max` is returned. For vectors
/// this operation is done component-wise.
pub fn clamp<T: HasClamp>(x: T, min: T::ElemType, max: T::ElemType) -> T {
	x.clamp(min, max)
}

/// Returns the linear interpolation of `x` and `y`. 
/// (i.e. `x * f + y * (1 - f)`). For vectors this is 
/// performed component-wise.
pub fn mix<T: HasMix>(x: T, y: T, f: T::ElemType) -> T {
	x.mix(y, f)
}

/// Returns 0.0 if `x` is smaller than `edge` and 1.0
/// otherwise. For vectors this operation is done 
/// component-wise.
pub fn step<T: HasStep>(edge: T, x: T) -> T {
	x.step(edge)
}

/// Returns 0.0 if `x` is smaller than `edge0` and 1.0
/// if `x` is larger then `edge1`. Otherwise the return
/// value is interpolated between 0.0 and 1.0 using 
/// Hermite polynomials. For vectors this operation is 
/// done component-wise.
pub fn smoothstep<T: HasSmoothStep>(edge0: T, edge1: T, x: T) -> T {
	x.smoothstep(edge0, edge1)
}

/// Returns the sine of the angle. For vectors this operation
/// is done component-wise.
pub fn sin<T: HasTrig>(x: T) -> T {
	x.sin()
}

/// Returns the cosine of `x`. For vectors this operation
/// is done component-wise.
pub fn cos<T: HasTrig>(x: T) -> T {
	x.cos()
}

/// Returns the tangent of `x`. For vectors this operation
/// is done component-wise.
pub fn tan<T: HasTrig>(x: T) -> T {
	x.tan()
}

/// Returns the arcsine of 'x'. For vectors this operation
/// is done component-wise.
pub fn asin<T: HasTrig>(x: T) -> T {
	x.asin()
}

/// Returns the arccosine of `x`. For vectors this operation
/// is done component-wise.
pub fn acos<T: HasTrig>(x: T) -> T {
	x.acos()
}

/// Returns the arctangent of `x`. For vectors this operation
/// is done component-wise.
pub fn atan<T: HasTrig>(x: T) -> T {
	x.atan()
}

/// Returns `x` raised to the power `y`. For vectors this
/// operation is done component-wise.
pub fn pow<T: HasExponential>(x: T, y: T) -> T {
	x.pow(y)
}

/// Returns the constant e raised to the power of `x`. For
/// vectors this operation is done component-wise.
pub fn exp<T: HasExponential>(x: T) -> T {
	x.exp()
}

/// Returns the natural logarithm of `x`.
pub fn log<T: HasExponential>(x: T) -> T {
	x.log()
}

/// Returns 2 raised to the power of `x`. For vectors
/// this operation is done component-wise.
pub fn exp2<T: HasExponential>(x: T) -> T {
	x.exp2()
}

/// Returns the logarithm base 2 of `x`. For vectors
/// this operation is done component-wise.
pub fn log2<T: HasExponential>(x: T) -> T {
	x.log2()
}

/// Returns the square root of `x`.
pub fn sqrt<T: HasSqrt>(x: T) -> T {
	x.sqrt()
}

/// Returns `1 / sqrt(x)`. In the future this function
/// may be faster than just evaluating `1 / sqrt(x)`.
/// For vectors this function is implemented component-wise.
pub fn inversesqrt<T: HasSqrt>(x: T) -> T {
	x.inv_sqrt()
}
