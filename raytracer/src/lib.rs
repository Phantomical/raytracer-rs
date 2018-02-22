
#![feature(conservative_impl_trait)]

extern crate glslvec;
extern crate gradient;

extern crate futures;
extern crate futures_cpupool;

#[macro_use]
extern crate ndarray;

mod vec;
mod array_vec;

mod types;
mod engine;

pub mod prelude;
pub mod object;
pub mod material;
