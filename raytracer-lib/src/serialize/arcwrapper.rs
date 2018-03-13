
#![allow(dead_code)]

use serde::{Deserialize, Deserializer};

use std::sync::Arc;
use object::Raymarchable;
use material::Material;
use light::Light;

use vec::Vec3d;
use serialize::{object, material, light};

#[derive(Serialize)]
pub struct RaymarchableWrapper(Arc<Raymarchable>);
//#[derive(Serialize)]
pub struct MaterialWrapper(Arc<Material>);
//#[derive(Serialize)]
pub struct LightWrapper(Arc<Light>);

impl Raymarchable for RaymarchableWrapper {
	fn distance(&self, point: Vec3d) -> f64 {
		self.0.distance(point)
	}

	fn normal_at(&self, point: Vec3d, dir: Vec3d) -> Vec3d {
		self.0.normal_at(point, dir)
	}
}

impl<'de> Deserialize<'de> for RaymarchableWrapper {
	fn deserialize<D>(de: D) -> Result<Self, D::Error> 
		where D: Deserializer<'de>
	{
		Ok(RaymarchableWrapper(try!(object::deserialize(de))))
	}
}
