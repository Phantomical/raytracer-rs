
use lib::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::sync::Arc;

pub fn build_scene(scenedesc : &str) -> Scene {
	unimplemented!();
}



pub enum TokenType {
	Object(String, HashMap<String, Box<TokenType>>),
	Number(f64),
	Vector(Vec<f64>)
}

pub trait Deserializer<T> {
	fn deserialize<'de>(&self, tok : &'de TokenType) -> T {
		<Self as Deserializer<T>>::deserialize_static(tok)
	}

	fn deserialize_static<'de>(tok : &'de TokenType) -> T;
}


