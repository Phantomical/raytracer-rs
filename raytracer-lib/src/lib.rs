#![feature(generators, generator_trait, conservative_impl_trait)]
#![feature(specialization)]

extern crate pbr;
extern crate rand;
extern crate glslvec;
extern crate gradient;
extern crate rhai;
extern crate image;
extern crate futures;
extern crate futures_cpupool;

pub mod object;
pub mod material;
pub mod colours;
pub mod light;
pub mod builder;

mod vec;
mod ray;
mod intersection;
mod raymarcher;
mod scene;
mod camera;
mod scenedata;
mod cacheable;
mod scenebuilder;
mod newscene;
mod scripting;

pub use vec::*;
pub use ray::*;
pub use intersection::*;
pub use raymarcher::*;
pub use scene::*;
pub use camera::*;
pub use image::*;
pub use scenedata::*;
pub use scenebuilder::*;
pub use newscene::*;

mod lib {
    pub use ::*;
}
