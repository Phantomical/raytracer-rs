
//! A vector math library which attempts to mostly replicate
//! the vector math functions available in GLSL.
//!
//! This means that all functions are free functions that
//! delegate their workings to an internal trait. If needed
//! those templates are available in the 
//! [prelude](prelude/index.html) module. They should only be
//! required if implementing new vector types. 

#![feature(slice_patterns)]

mod traits;
mod vec2;
mod vec3;
mod vec4;
mod trait_impls;
mod functions;
mod float_mul;

mod mat3;

pub mod prelude {
	//! Traits that are used to provide the implementations
	//! for free function in the library. These should not
	//! be necessary to use unless implementing custom 
	//! vector types or writing generic functions.

	pub use traits::*;
	pub use trait_impls::*;
}

pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use functions::*;
pub use mat3::*;

mod swizzle;
pub use swizzle::*;
