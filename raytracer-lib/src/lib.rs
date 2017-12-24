
pub mod object;
pub mod material;

mod vec;
mod ray;

pub use vec::*;
pub use ray::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
