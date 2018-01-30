#![feature(generators, generator_trait, conservative_impl_trait)]
#![feature(specialization)]

extern crate num_cpus;
extern crate pbr;
extern crate rand;
extern crate threadpool;
extern crate serde;
extern crate glslvec;
extern crate gradient;

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
mod image;
mod scenedata;

pub use vec::*;
pub use ray::*;
pub use intersection::*;
pub use raymarcher::*;
pub use scene::*;
pub use camera::*;
pub use image::*;
pub use scenedata::*;

mod lib {
    pub use ::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
