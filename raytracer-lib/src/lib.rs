
pub mod object;
pub mod material;

mod vec;
mod ray;
mod intersection;
mod raymarcher;

pub use vec::*;
pub use ray::*;
pub use intersection::*;
pub use raymarcher::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
