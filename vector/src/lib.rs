
#![feature(specialization)]
#![feature(const_fn)]

extern crate num;

mod traits;
mod vecmacro;
mod functions;

pub mod prelude {
	pub use traits::*;
}

pub use functions::*;
pub use vecmacro::*;

#[cfg(feature = "colour")]
mod colour;
#[cfg(feature = "colour")]
pub mod colours;

#[cfg(feature = "colour")]
pub use colour::Colour;
