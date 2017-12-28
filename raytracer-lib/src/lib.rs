
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

pub use vec::*;
pub use ray::*;
pub use intersection::*;
pub use raymarcher::*;
pub use scene::*;
pub use camera::*;
pub use image::*;

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
