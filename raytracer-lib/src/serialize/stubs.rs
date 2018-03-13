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
				$tyname($type)
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
						}
					),*
				}
			}
		}
	}
}

#[allow(dead_code)]
pub mod object {
    use object::Raymarchable;
    use serde::Deserializer;
    use std::sync::Arc;
    use object;

    deserialization_table!(Raymarchable, [
		(object::Sphere, Sphere, "sphere"),
		//(object::BoundSphere<Arc<Raymarchable>>, BoundSphere, "boundsphere"),
		(object::BoxObj, BoxObj, "box"),
		(object::Cone, Cone, "cone"),
		(object::Cylinder, Cylinder, "cylinder"),
		(object::HexagonalPrism, HexagonalPrism, "hexagonal_prism"),
		//(object::Hollow<Arc<Raymarchable>>, Hollow, "hollow"),
		(object::Mandelbulb, Mandelbulb, "mandelbulb"),
		(object::Plane, Plane, "plane"),
		//(object::Repeat<Arc<Raymarchable>>, Repeat, "repeat"),
		//(object::Rotate<Arc<Raymarchable>>, Rotate, "rotate"),
		(object::Sierpinski, Sierpinski, "sierpinski"),
		(object::Torus, Torus, "torus"),
		//(object::Transform<Arc<Raymarchable>>, Transform, "transform"),
		//(object::Translate<Arc<Raymarchable>>, Translate, "translate"),
		(object::TriangularPrism, TriangularPrism, "triangular_prism")
	]);

    pub fn deserialize<'de, D>(d: D) -> Result<Arc<Raymarchable>, D::Error>
    where
        D: Deserializer<'de>,
    {
        DeserializationTable::deserialize_to_arc(d)
    }
}
#[allow(dead_code)]
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
#[allow(dead_code)]
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
