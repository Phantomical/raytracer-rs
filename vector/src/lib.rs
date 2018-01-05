
#![feature(specialization)]

extern crate num;

mod traits;
mod vecmacro;
mod functions;
mod traitimpls;

pub mod prelude {
	pub use traits::*;
}

pub use functions::*;
pub use vecmacro::*;
pub use traitimpls::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
