
use lib::*;

use parser::scenebuilder::*;

pub struct VecDeserializer {}
impl Deserializer<Vec3d> for VecDeserializer {
	fn deserialize_static<'de>(tok : &'de TokenType) -> Vec3d {
		match tok {
			&TokenType::Vector(ref vec) => {
				if vec.len() != 3 { panic!(); }
				Vec3d::new(vec[0], vec[1], vec[2])
			},
			_ => panic!("Expected Vector, found other type")
		}
	}
}