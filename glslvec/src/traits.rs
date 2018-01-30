
use std::ops::Fn;

/// Provides a dot operation which is used by the 
/// [`dot`](../fn.dot.html) function.
pub trait HasDot {
	/// The resulting type of the dot product.
	type Output;

	/// Returns the dot product of two vectors.
	fn dot(&self, other: Self) -> Self::Output;
}
/// Provices a cross operation which is used by the
/// [`cross`](../fn.cross.html) function.
pub trait HasCross {
	/// Returns the cross product of two vectors.
	fn cross(&self, other: Self) -> Self;
}
/// Provides the length method for use by the 
/// [`length`](../fn.length.html) function.
pub trait HasLength {
	/// The type of the length of the vector.
	type Output;

	/// Returns the length of the vector.
	fn length(&self) -> Self::Output;

	#[cfg(magnitude)]
	/// This is an alias for the `length` function.
	fn magnitude(&self) -> Self::Output {
		return self.length();
	}
}
/// Provides the distance operation for use by
/// the [`distance`](../fn.distance.html) function.
pub trait HasDistance {
	/// The resulting type from taking the distnace
	/// between two vectors.
	type Output;

	/// Returns the distance between two vectors.
	fn distance(&self, rhs: Self) -> Self::Output;
}
/// Provides the normalize operation for use by
/// the [`normalize`](../fn.normalize.html) function.
pub trait HasNormalize {
	/// Returns a parallel vector with magnitude 1.
	fn normalize(&self) -> Self;
}

/// Provides the faceforward operation for use by
/// the [`faceforward`](../fn.normalize.html) function.
pub trait HasFaceForward {
	fn faceforward(&self, incident: Self, nref: Self) -> Self;
}
/// Provides the reflect operation for use by the
/// [`reflect`](../fn.reflect.html) function.
pub trait HasReflect {
	fn reflect(&self, normal: Self) -> Self;
}
/// Provides the refract operation for use by the 
/// [`refract`](../fn.refract.html) function.
pub trait HasRefract {
	type ElemType;

	fn refract(&self, normal: Self, eta: Self::ElemType) -> Self;
}

/// A trait that exposes an interface to apply an operation 
/// to each component of the vector. This trait is used to
/// automatically implement several other traits if the 
/// element type supports the desired operation.
pub trait HasPerElementOps {
	/// The type of the elements of the vector.
	type ElemType;

	/// Applies an operation to each component of the vector
	/// and returns a vector of the results.
	fn apply_op<T:Fn(&Self::ElemType) -> Self::ElemType>(&self, func: T) -> Self;
}

/// A trait that exposes an interface to apply a binary
/// operation to each component of the vector. This trait
/// is used to automatically implement several other traits
/// if the element type supports the desired operation.
pub trait HasPerElementBinOps {
	/// The type of the elements of the vector.
	type ElemType;

	/// Applies a binary operation to each component of 
	/// the vectors and returns the results as a vector.
	fn apply_bin_op<T:Fn(&Self::ElemType, Self::ElemType)->Self::ElemType>(&self, rhs: Self, func: T) -> Self;
}

/// A trait that exposes the abs operation for use
/// by the [`abs`](../fn.abs.html) function.
pub trait HasAbs {
	/// Returns the component-wise absolute value of a vector.
	fn abs(&self) -> Self;
}

/// A trait that exposes useful trig functions.
pub trait HasTrig {
	/// Returns the component-wise sine of a vector.
	fn sin(&self) -> Self;
	/// Returns the component-wise cosine of a vector.
	fn cos(&self) -> Self;
	/// Returns the component-wise tangent of a vector.
	fn tan(&self) -> Self;

	/// Returns the component-wise arcsine of a vector.
	fn asin(&self) -> Self;
	/// Returns the component-wise arccosine of a vector.
	fn acos(&self) -> Self;
	/// Returns the component-wise arctangent of a vector.
	fn atan(&self) -> Self;
}

/// A trait that exposes exponential functions.
pub trait HasExponential {
	/// Returns `x^exponent` for each component in a vector.
	fn pow(&self, exponent: Self) -> Self;
	/// Returns the component-wise natural logarithm of a vector.
	fn log(&self) -> Self;
	/// Return `e^x` for each component `x` within a vector.
	fn exp(&self) -> Self;
	/// Returns the component-wise base 2 logarithm of a vector.
	fn log2(&self) -> Self;
	/// Returns `2^x` for each component `x` within a vector.
	fn exp2(&self) -> Self;
}

/// A trait that exposes the square root operation.
pub trait HasSqrt {
	/// Returns the component-wise square root of a vector.
	fn sqrt(&self) -> Self;
	/// Return the component-wise inverse square root of a vector.
	fn inv_sqrt(&self) -> Self;
}

/// Exposes the sign operation for use by the 
/// [`sign`](../fn.sign.html) function.
pub trait HasSign {
	/// Returns -1.0 if negative, 1.0 if positive and 0.0 otherwise.
	fn sign(&self) -> Self;
}

/// Exposes the floor operation for use by the 
/// [`floor`](../fn.floor.html) function.
pub trait HasFloor {
	/// Returns the component-wise floor of a vector.
	fn floor(&self) -> Self;
}
/// Exposes the ceil operation for use by the 
/// [`ceil`](../fn.ceil.html) function.
pub trait HasCeil {
	/// Returns the component-wise ceil of a vector.
	fn ceil(&self) -> Self;
}
/// Exposes the fract operation for use by the
/// [`fract`](../fn.fract.html) function.
pub trait HasFract {
	/// Returns the fractional part of each component of a vector.
	fn fract(&self) -> Self;
}

/// Exposes min and max operations for vectors.
pub trait HasMinMax {
	/// Returns the component-wise minimum between two vectors.
	fn min(&self, rhs: Self) -> Self;
	/// Returns the component-wise maximum between two vectors.
	fn max(&self, rhs: Self) -> Self;
}

/// Exposes a clamp operation for vectors.
pub trait HasClamp {
	type ElemType;

	fn clamp(&self, min: Self::ElemType, max: Self::ElemType) -> Self;
}

/// Exposes the mix operation (linear interpolation)
/// for use by the [`mix`](../fn.mix.html) function.
pub trait HasMix {
	/// The type of the interpolation parameter.
	type ElemType;

	/// Returns the linear interpolation between two
	/// vectors as determined by the `f` parameter.
	fn mix(&self, rhs: Self, f: Self::ElemType) -> Self;
}

/// Exposes the step operation for use by the
/// [`step`](../fn.step.html) function.
pub trait HasStep {
	/// For each component, returns 0.0 if less than edge, 1.0 otherwise.
	/// NOTE: GLSL function arguments are the other way around.
	fn step(&self, edge: Self) -> Self;
}

/// Exposes the smooth step operation for use by the
/// [`smoothstep`](../fn.smoothstep.html) function.
pub trait HasSmoothStep {
	/// Returns 0.0 if less than edge0, 1.0 if greater than
	/// edge1, and uses Hermite interpolation between the
	/// values otherwise.
	fn smoothstep(&self, edge0: Self, edge1: Self) -> Self;
}

pub trait Zero {
	fn zero() -> Self;
}
pub trait One {
	fn one() -> Self;
}
