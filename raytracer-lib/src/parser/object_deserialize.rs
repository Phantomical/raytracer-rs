
use lib::*;
use lib::object::*;

use parser::scenebuilder::*;
use parser::vec_deserialize::*;
use std::sync::Arc;

macro_rules! make_deserializer {
	($name:ident, $type:ty, $arg:ident, $func:expr) => {
		pub struct $name {}
		impl Deserializer<$type> for $name {
			fn deserialize_static<'de>($arg : &'de TokenType) -> $type {
				$func
			}
		}
	}
}



make_deserializer!(BoxDeserializer, Arc<Raymarchable>, tok, {
	match tok {
		&TokenType::Object(_, ref items) => {
			Arc::new(Box {
				bounds: VecDeserializer::deserialize_static(&items["bounds"])
			})
		},
		_ => panic!("Unexpected type!")
	}
});
make_deserializer!(SphereDeserializer, Arc<Raymarchable>, tok, {
	match tok {
		&TokenType::Object(_, ref items) => {
			Arc::new(Sphere {
				radius: match *items["radius"] {
					TokenType::Number(n) => n,
					_ => panic!("Invalid radius type")
				},
				centre: VecDeserializer::deserialize_static(&items["bounds"])
			})
		},
		_ => panic!("Unexpected type!")
	}
});
