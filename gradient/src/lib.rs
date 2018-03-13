
extern crate num;

#[cfg(feature="serde")]
#[cfg_attr(feature="serde", macro_use)]
extern crate serde;

mod gradient;

pub use gradient::*;
