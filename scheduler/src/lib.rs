
#![feature(generators, generator_trait)]
#![feature(refcell_replace_swap)]

extern crate multimap;

mod scheduler;

pub use scheduler::*;

mod lib {
	//pub use ::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
