

use lib::*;
use std::result::Result;

use serde_json::*;

pub enum DeserializeError<'de> {
	VectorWrongSize (&'de Value),
	WrongType       (&'de Value),
	NumberOutOfRange(&'de Value)
}

pub trait Deserialize: Sized {
	fn deserialize<'de>(val : &'de Value) -> Result<Self, DeserializeError>;
}

pub trait Deserializer<T> {
	fn deserialize<'de>(val : &'de Value) -> Result<T, DeserializeError>;
}

impl<T> Deserializer<T> for T
	where T: Deserialize
{
	fn deserialize<'de>(val : &'de Value) -> Result<T, DeserializeError> {
		T::deserialize(val)
	}
}

impl Deserializer<f64> for f64 {
	fn deserialize<'de>(val : &'de Value) -> Result<f64, DeserializeError> {
		match *val {
			Value::Number(ref n) => {
				if let Some(v) = n.as_f64() {
					Ok(v)
				}
				else {
					Err(DeserializeError::NumberOutOfRange(val))
				}
			}
			_ => Err(DeserializeError::WrongType(val))
		}
	}
}
impl Deserializer<f32> for f32 {
	fn deserialize<'de>(val : &'de Value) -> Result<f32, DeserializeError> {
		match *val {
			Value::Number(ref n) => {
				if let Some(v) = n.as_f64() {
					Ok(v as f32)
				}
				else {
					Err(DeserializeError::NumberOutOfRange(val))
				}
			}
			_ => Err(DeserializeError::WrongType(val))
		}
	}
}

impl Deserializer<Vec3d> for Vec3d {
	fn deserialize<'de>(val : &'de Value) -> Result<Vec3d, DeserializeError> {
		match *val {
			Value::Array(ref vec) => {
				if vec.len() != 3 {
					Err(DeserializeError::VectorWrongSize(val))
				}
				else {
					let x = f64::deserialize(&vec[0]);
					let y = f64::deserialize(&vec[1]);
					let z = f64::deserialize(&vec[2]);
					
					if x.is_err()      { Err(x.err().unwrap()) }
					else if y.is_err() { Err(y.err().unwrap()) }
					else if z.is_err() { Err(z.err().unwrap()) }
					else {
						Ok(Vec3d::new(
							x.ok().unwrap(),
							y.ok().unwrap(),
							z.ok().unwrap()
						))
					}
				}
			}
			_ => Err(DeserializeError::WrongType(val))
		}
	}
}



