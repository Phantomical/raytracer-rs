#![feature(generators, generator_trait, conservative_impl_trait)]
#![feature(specialization)]

extern crate num_cpus;
extern crate pbr;
extern crate rand;
extern crate threadpool;
extern crate glslvec;
extern crate gradient;
extern crate rhai;
extern crate image;

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
