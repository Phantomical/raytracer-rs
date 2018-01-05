
#![feature(specialization)]
#![feature(const_fn)]

extern crate num;

mod traits;
mod typedefs;
mod vecmacro;
mod functions;

pub mod prelude {
	pub use traits::*;
}

pub use functions::*;
pub use vecmacro::*;
pub use typedefs::*;

#[cfg(feature = "colour")]
mod colour;
#[cfg(feature = "colour")]
pub mod colours;

#[cfg(feature = "colour")]
pub use colour::Colour;
