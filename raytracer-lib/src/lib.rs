
pub mod object;
pub mod material;

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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
