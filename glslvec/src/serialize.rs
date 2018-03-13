
use serde::*;
use vec2::*;
use vec3::*;
use vec4::*;

impl<T: Serialize + Clone> Serialize for Vec2<T> {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> 
		where S: Serializer
	{
		[self.x.clone(), self.y.clone()]
			.serialize(s)
	}
}
impl<T: Serialize + Clone> Serialize for Vec3<T> {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> 
		where S: Serializer
	{
		[self.x.clone(), self.y.clone(), self.z.clone()]
			.serialize(s)
	}
}
impl<T: Serialize + Clone> Serialize for Vec4<T> {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> 
		where S: Serializer
	{
		[self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone()]
			.serialize(s)
	}
}

impl<'de, T: Deserialize<'de> + Clone> Deserialize<'de> for Vec2<T> {
	fn deserialize<D>(de: D) -> Result<Vec2<T>, D::Error>
		where D: Deserializer<'de>
	{
		let [x, y] = try!(<[T;2]>::deserialize(de));
		Ok(vec2(x, y))
	}
}
impl<'de, T: Deserialize<'de> + Clone> Deserialize<'de> for Vec3<T> {
	fn deserialize<D>(de: D) -> Result<Self, D::Error>
		where D: Deserializer<'de>
	{
		let [x, y, z] = try!(<[T;3]>::deserialize(de));
		Ok(vec3(x, y, z))
	}
}
impl<'de, T: Deserialize<'de> + Clone> Deserialize<'de> for Vec4<T> {
	fn deserialize<D>(de: D) -> Result<Self, D::Error>
		where D: Deserializer<'de>
	{
		let [x, y, z, w] = try!(<[T;4]>::deserialize(de));
		Ok(vec4(x, y, z, w))
	}
}
