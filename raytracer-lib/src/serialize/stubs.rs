use erased_serde::Deserializer;
use std::sync::Arc;

pub trait TypeDeserializer<T: ?Sized>: Sync {
    fn deserialize<'de>(&self, v: &mut Deserializer<'de>) -> Arc<T>;
}

macro_rules! deserialization_table {
	($trait:ty, [$(($type:ty, $tyname:ident, $name:tt)),* ]) => {
		#[derive(Deserialize)]
		#[serde(tag = "type")]
		enum DeserializationTable
		{
			$(
				#[serde(rename = $name)]
				$tyname($type),
			),*
		}

		impl DeserializationTable {
			pub fn deserialize_to_arc<'de, D>(d: D)
					-> Result<::std::sync::Arc<$trait>, D::Error>
				where D: ::serde::Deserializer<'de>
			{
				#[allow(unused_imports)]
				use std::sync::Arc;
				use serde::Deserialize;

				match try!(Self::deserialize(d)) {
					$(
						DeserializationTable::$tyname(val) => {
							let a: Arc<$type> = Arc::new(val);
							Ok(a)
						},
					),*
				}
			}
		}
	}
}

pub mod object {
    use object::Object;
    use serde::Deserializer;
    use std::sync::Arc;
    use object;

    deserialization_table!(Raymarchable, [
		(object::Sphere, Sphere, "sphere")
	]);

    pub fn deserialize<'de, D>(d: D) -> Result<Arc<Object>, D::Error>
    where
        D: Deserializer<'de>,
    {
        DeserializationTable::deserialize_to_arc(d)
    }
}
pub mod material {
    use material::Material;
    use serde::Deserializer;
    use std::sync::Arc;
    use material;

    deserialization_table!(Material, [(material::SolidColour, SolidColour, "colour")]);

    pub fn deserialize<'de, D>(d: D) -> Result<Arc<Material>, D::Error>
    where
        D: Deserializer<'de>,
    {
        DeserializationTable::deserialize_to_arc(d)
    }
}
pub mod light {
    use light::Light;
    use serde::Deserializer;
    use std::sync::Arc;
    #[allow(unused_imports)]
    use light;

    deserialization_table!(Light, []);

    pub fn deserialize<'de, D>(d: D) -> Result<Arc<Light>, D::Error>
    where
        D: Deserializer<'de>,
    {
        DeserializationTable::deserialize_to_arc(d)
    }
}
