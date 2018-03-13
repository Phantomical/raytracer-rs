
macro_rules! type_serialization_decl {
	($name:tt, $module:ident) => {
		mod $module {
			use serde::{Serializer, Deserializer};

			#[allow(dead_code)]
			pub fn serialize<S>(_: &(), s: S) -> Result<S::Ok, S::Error>
				where S: Serializer
			{
				s.serialize_str($name)
			}

			#[allow(dead_code)]
			pub fn deserialize<'de, D>(_: D) -> Result<(), D::Error>
				where D: Deserializer<'de>
			{
				unimplemented!();
			}
		}
	};
	($name:tt) => {
		type_serialization_decl!($name, tag);
	}
}
