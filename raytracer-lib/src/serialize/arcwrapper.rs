#![allow(dead_code)]

use serde::{Deserialize, Deserializer};

use std::sync::Arc;
use object::Raymarchable;
use material::Material;
use light::Light;

use vec::{Colour, Vec3d};
use lib::{Intersection, Ray};
use serialize::{light, material, object};

#[derive(Serialize)]
pub struct RaymarchableWrapper(Arc<Raymarchable>);
#[derive(Serialize)]
pub struct MaterialWrapper(Arc<Material>);
#[derive(Serialize)]
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
    where
        D: Deserializer<'de>,
    {
        Ok(RaymarchableWrapper(try!(object::deserialize(de))))
    }
}

impl Material for MaterialWrapper {
    fn base_colour(&self, isect: &Intersection) -> Colour {
        self.0.base_colour(isect)
    }
    fn roughness(&self, isect: &Intersection) -> f32 {
        self.0.roughness(isect)
    }
    fn reflectivity(&self, isect: &Intersection) -> f32 {
        self.0.reflectivity(isect)
    }
}

impl<'de> Deserialize<'de> for MaterialWrapper {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MaterialWrapper(try!(material::deserialize(de))))
    }
}

impl Light for LightWrapper {
    fn illumination(&self, isect: &Intersection) -> Colour {
        self.0.illumination(isect)
    }
    fn shadow_rays(&self, isect: &Intersection) -> Box<Iterator<Item = (Ray, f64)>> {
        self.0.shadow_rays(isect)
    }
}

impl<'de> Deserialize<'de> for LightWrapper {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LightWrapper(try!(light::deserialize(de))))
    }
}
