
#![feature(specialization)]

extern crate num;

mod traits;
mod vecmacro;
mod functions;

pub mod prelude {
	pub use traits::*;
}

pub use functions::*;
pub use vecmacro::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
