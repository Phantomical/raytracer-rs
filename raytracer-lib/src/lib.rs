//! Raytracer

#![feature(generators, generator_trait, conservative_impl_trait)]
#![feature(specialization)]

extern crate futures;
extern crate futures_cpupool;
extern crate glslvec;
extern crate gradient;
extern crate image;
extern crate pbr;
extern crate rand;

#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate serde;

#[macro_use]
mod serialize;

pub mod object;
pub mod material;
pub mod colours;
pub mod light;
pub mod builder;

mod vec;
mod ray;
mod intersection;
mod camera;
mod cacheable;
mod scenebuilder;
mod newscene;
mod engine;

pub use ray::*;
pub use intersection::*;
pub use camera::*;
pub use image::*;
pub use scenebuilder::*;
pub use newscene::*;
pub use engine::*;

pub mod prelude {
	pub use object::{Raymarchable, IFS, Analytical};
	pub use material::Material;
	pub use light::Light;
}

pub mod math {
	pub use vec::*;
}

pub(crate) use math::*;

mod lib {
    pub use ::*;
}

