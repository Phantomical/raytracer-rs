
#![feature(generators, generator_trait, conservative_impl_trait)]
#![feature(specialization)]

extern crate rand;
extern crate num_cpus;
extern crate threadpool;

extern crate serde_json;

pub mod object;
pub mod material;
pub mod colours;
pub mod light;

mod vec;
mod ray;
mod intersection;
mod raymarcher;
mod scene;
mod camera;
mod image;
mod scenedata;
mod serialization;

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
